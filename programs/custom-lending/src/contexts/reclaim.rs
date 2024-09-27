use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{error::ErrorCode, state::{Init, Listing}, helpers::transfer};

#[derive(Accounts)] 
pub struct Reclaim<'info> {
    #[account(mut)]
    closer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), listing.maker.key().as_ref(), listing.seed.to_le_bytes().as_ref()],
        bump,
    )]
    listing: Account<'info, Listing>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Account<'info, Init>,
    #[account(
        mut,
        seeds = [b"sol_vault", listing.maker.key().as_ref(), listing.seed.to_le_bytes().as_ref()],
        bump = listing.vault_bump,
    )]
    maker_sol_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        seeds = [b"sol_vault", listing.taker.key().as_ref(), listing.seed.to_le_bytes().as_ref()],
        bump = listing.vault_bump,
    )]
    taker_sol_vault: Option<SystemAccount<'info>>,
    mint_b: Option<InterfaceAccount<'info, Mint>>,
    #[account(
        associated_token::authority = listing,
        associated_token::mint = mint_b,
    )]
    vault_b: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::authority = closer,
        associated_token::mint = mint_b,
    )]
    taker_mint_ata_b: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasure_bump,
    )]
    treasury: SystemAccount<'info>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Reclaim<'info> {
    pub fn reclaim(&mut self) -> Result<()> {
        let (lender, vault) = match self.listing.listing_type {
            true => {
                (self.listing.maker, self.maker_sol_vault.as_ref().unwrap().to_account_info())
            },
            false => {
                (self.listing.maker, self.taker_sol_vault.as_ref().unwrap().to_account_info())
            }
        };

        require!(self.listing.active, ErrorCode::NotActive);
        require!(lender != Pubkey::default() && lender == self.closer.key(), ErrorCode::Unauthorised);
        require!(self.listing.creation_timestamp + self.listing.repay_time >= Clock::get()?.unix_timestamp, ErrorCode::CannotRepay);

        let is_collat_sol = self.listing.collateral_token == self.system_program.key();

        let seed = self.listing.seed.to_le_bytes();
        let bump = [self.listing.bump];
        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.listing.maker.key().to_bytes()[..],
            &seed.as_ref(),
            &bump,
        ];
        let signer_seeds = &[&seeds[..]];

        if is_collat_sol {
            transfer::transfer_sol(self.system_program.to_account_info(), signer_seeds, self.closer.to_account_info(), vault, self.listing.collateral_amount)?;
        } else {
            transfer::transfer_spl(self.token_program.to_account_info(), signer_seeds, self.mint_b.as_ref().unwrap().clone(), self.taker_mint_ata_b.as_ref().unwrap().to_account_info(), self.vault_b.as_ref().unwrap().to_account_info(), self.listing.to_account_info(), self.listing.collateral_amount)?;
            transfer::close_spl_account(self.token_program.to_account_info(), signer_seeds, self.vault_b.as_ref().unwrap().to_account_info(), self.treasury.to_account_info(), self.listing.to_account_info())?;
        }
        Ok(())
    }

}