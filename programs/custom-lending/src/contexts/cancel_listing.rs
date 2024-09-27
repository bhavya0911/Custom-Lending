use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{error::ErrorCode, state::{Init, Listing}, helpers::transfer};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CancelListing<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        mut,
        close = treasury,
        seeds = [b"listing", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = listing.bump,
    )]
    listing: Account<'info, Listing>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Account<'info, Init>,
    mint: Option<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = mint,
    )]
    maker_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = mint,
    )]
    spl_vault: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"sol_vault", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = listing.vault_bump,
    )]
    sol_vault: Option<SystemAccount<'info>>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasure_bump,
    )]
    treasury: SystemAccount<'info>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> CancelListing<'info> {
    pub fn cancel(&mut self) -> Result<()> {
        require!(self.listing.taker == Pubkey::default(), ErrorCode::AlreadyTaken);
        require!(self.listing.active, ErrorCode::NotActive);
        let listing = &self.listing;

        let (amount, token) = match listing.listing_type {
            true => {
                (listing.loan_amount, listing.loan_token)
            },
            false => {
                (listing.collateral_amount, listing.collateral_token)
            },
        };

        let is_sol = token.key() == self.system_program.to_account_info().key();

        let seed = self.listing.seed.to_le_bytes();
        let bump = [self.listing.bump];
        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.maker.key().to_bytes()[..],
            &seed.as_ref(),
            &bump,
        ];
        let signer_seeds = &[&seeds[..]];

        if is_sol {
            transfer::transfer_sol(self.system_program.to_account_info(), signer_seeds, self.maker.to_account_info(), self.sol_vault.as_ref().unwrap().to_account_info(), amount)?;
        } else {
            transfer::transfer_spl(self.token_program.to_account_info(), signer_seeds, self.mint.as_ref().unwrap().clone(), self.maker_ata.as_ref().unwrap().to_account_info(), self.spl_vault.as_ref().unwrap().to_account_info(), self.listing.to_account_info(), amount)?;
            transfer::close_spl_account(self.token_program.to_account_info(), signer_seeds, self.spl_vault.as_ref().unwrap().to_account_info(), self.treasury.to_account_info(), self.listing.to_account_info())?;
        }

        Ok(())
    }
}