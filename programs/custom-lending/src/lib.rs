use anchor_lang::prelude::*;

mod constants;
mod state;
mod contexts;
use contexts::*;
mod error;
mod helpers;

declare_id!("6upffmi5mugYXGXioZMc2mQeqjnPJX9rkPqwMGDSycJP");

#[program]
pub mod capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        ctx.accounts.init(name, &ctx.bumps)
    }

    pub fn create(ctx: Context<CreateListing>, seed: u64, listing_type: bool, collateral_token: Pubkey, collateral_amount: u64, loan_token: Pubkey, loan_amount: u64, expiry: i64, repay_time: i64) -> Result<()> {
        ctx.accounts.create(&ctx.bumps, seed, listing_type, collateral_token, collateral_amount, loan_token, loan_amount, expiry, repay_time)
    }

    pub fn accept(ctx: Context<AcceptListing>) -> Result<()> {
        ctx.accounts.accept_listing()
    }

    pub fn cancel(ctx: Context<CancelListing>) -> Result<()> {
        ctx.accounts.cancel()
    }

    pub fn repay(ctx: Context<Repay>) -> Result<()> {
        ctx.accounts.repay()
    }

    pub fn reclaim(ctx: Context<Reclaim>) -> Result<()> {
        ctx.accounts.reclaim()
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }
}