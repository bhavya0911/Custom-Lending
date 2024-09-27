use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface},
    associated_token::AssociatedToken
};

use crate::{state::{Init, Listing}, helpers::transfer};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateListing<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [b"listing", marketplace.key().as_ref(), maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
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
        init_if_needed,
        payer = maker,
        associated_token::authority = listing,
        associated_token::mint = mint,
    )]
    spl_vault: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"sol_vault", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    sol_vault: Option<SystemAccount<'info>>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateListing<'info> {
    pub fn create(&mut self, bump: &CreateListingBumps, seed: u64, listing_type: bool, collateral_token: Pubkey, collateral_amount: u64, loan_token: Pubkey, loan_amount: u64, expiry: i64, repay_time: i64) -> Result<()> {
        let (amount, token) = match listing_type {
            true => {
                (loan_amount, loan_token)
            },
            false => {
                (collateral_amount, collateral_token)
            },
        };

        let (is_sol, vault_bump) = match token.key() == self.system_program.to_account_info().key() {
            true => {
                (true, bump.sol_vault.unwrap())
            },
            false => {
                (false, 0)
            },
        };

        self.listing.set_inner(Listing { 
            bump: bump.listing,
            vault_bump, 
            seed,
            maker: self.maker.key(), 
            taker: Pubkey::default(),
            listing_type, 
            collateral_token, 
            collateral_amount,
            loan_token, 
            loan_amount, 
            creation_timestamp: Clock::get()?.unix_timestamp,
            active: true, 
            expiry,
            repay_time,
        });

        if is_sol {
            transfer::accept_sol(self.system_program.to_account_info(), self.sol_vault.as_ref().unwrap().to_account_info(), self.maker.to_account_info(), amount)?;
        } else {
            transfer::accept_spl(self.token_program.to_account_info(), self.mint.as_ref().unwrap().clone(), self.spl_vault.as_ref().unwrap().to_account_info(), self.maker_ata.as_ref().unwrap().to_account_info(), self.maker.to_account_info(), amount)?;
        }

        Ok(())
    }
}