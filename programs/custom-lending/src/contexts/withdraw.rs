use anchor_lang::{system_program::{transfer, Transfer}, prelude::*};

use crate::{state::Init, error::ErrorCode};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump,
    )]
    marketplace: Account<'info, Init>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    treasury: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require_keys_eq!(self.admin.key(), self.marketplace.admin, ErrorCode::Unauthorised);

        let seeds = &[
            b"marketplace",
            self.marketplace.name.as_str().as_bytes(),
        ];
        let signer_seeds = &[&seeds[..]];

        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.admin.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        transfer(ctx, self.treasury.lamports())
    }
}