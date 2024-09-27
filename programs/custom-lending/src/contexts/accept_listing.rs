use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{error::ErrorCode, state::{Init, Listing}, helpers::transfer};

#[derive(Accounts)]
pub struct AcceptListing<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    /// CHECK: this is safe
    #[account(mut)]
    maker: Option<UncheckedAccount<'info>>,
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
        seeds = [b"sol_vault", taker.key().as_ref(), listing.seed.to_le_bytes().as_ref()],
        bump = listing.vault_bump,
    )]
    taker_sol_vault: Option<SystemAccount<'info>>,
    #[account(
        mut,
        seeds = [b"sol_vault", listing.maker.key().as_ref(), listing.seed.to_le_bytes().as_ref()],
        bump = listing.vault_bump,
    )]
    maker_sol_vault: Option<SystemAccount<'info>>,
    mint_a: Option<InterfaceAccount<'info, Mint>>,
    #[account(
        associated_token::authority = listing,
        associated_token::mint = mint_a,
    )]
    vault_a: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::authority = taker,
        associated_token::mint = mint_a,
    )]
    taker_mint_ata_a: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::authority = maker,
        associated_token::mint = mint_a,
    )]
    maker_mint_ata_a: Option<InterfaceAccount<'info, TokenAccount>>,
    mint_b: Option<InterfaceAccount<'info, Mint>>,
    #[account(
        associated_token::authority = listing,
        associated_token::mint = mint_b,
    )]
    vault_b: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::authority = taker,
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

impl<'info> AcceptListing<'info> {
    pub fn accept_listing(&mut self) -> Result<()> {
        require!(self.listing.active, ErrorCode::NotActive);
        require!(self.listing.taker == Pubkey::default(), ErrorCode::AlreadyTaken);
        require!(self.listing.expiry >= Clock::get()?.unix_timestamp, ErrorCode::Expired);

        let (is_loan_sol, is_collat_sol) = (self.listing.loan_token.key() == self.system_program.key(), self.listing.collateral_token.key() == self.system_program.key());

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

        if self.listing.listing_type {
            if is_loan_sol {
                transfer::transfer_sol(self.system_program.to_account_info(), signer_seeds, self.taker.to_account_info(), self.maker_sol_vault.as_ref().unwrap().to_account_info(), self.listing.loan_amount)?;
            } else {
                transfer::transfer_spl(self.token_program.to_account_info(), signer_seeds, self.mint_a.as_ref().unwrap().clone(), self.taker_mint_ata_a.as_ref().unwrap().to_account_info(), self.vault_a.as_ref().unwrap().to_account_info(), self.listing.to_account_info(), self.listing.loan_amount)?;
                transfer::close_spl_account(self.token_program.to_account_info(), signer_seeds, self.vault_a.as_ref().unwrap().to_account_info(), self.treasury.to_account_info(), self.listing.to_account_info())?;
            }

            if is_collat_sol {
                transfer::accept_sol(self.system_program.to_account_info(), self.taker_sol_vault.as_ref().unwrap().to_account_info(), self.taker.to_account_info(), self.listing.collateral_amount)?;
            } else {
                transfer::accept_spl(self.token_program.to_account_info(), self.mint_b.as_ref().unwrap().clone(), self.vault_b.as_ref().unwrap().to_account_info(), self.taker_mint_ata_b.as_ref().unwrap().to_account_info(), self.taker.to_account_info(), self.listing.collateral_amount)?;
            }
        } else {
            if is_loan_sol {
                transfer::accept_sol(self.system_program.to_account_info(), self.taker.to_account_info(), self.maker.as_ref().unwrap().to_account_info(), self.listing.loan_amount)?;
            } else {
                transfer::accept_spl(self.token_program.to_account_info(), self.mint_a.as_ref().unwrap().clone(), self.maker_mint_ata_a.as_ref().unwrap().to_account_info(), self.taker_mint_ata_a.as_ref().unwrap().to_account_info(), self.taker.to_account_info(), self.listing.loan_amount)?;
            }
        }

        self.listing.taker = self.taker.key();
        self.listing.creation_timestamp = Clock::get()?.unix_timestamp;

        Ok(())
    }
}