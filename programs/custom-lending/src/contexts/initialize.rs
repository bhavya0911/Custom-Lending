use anchor_lang::prelude::*;

use crate::{state::Init, error::ErrorCode};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        space = Init::INIT_SPACE,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
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

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, bumps: &InitializeBumps) -> Result<()> {
        require!(name.len() > 0 && name.len() < 33, ErrorCode::NameTooLong);

        self.marketplace.set_inner(Init { 
            admin: self.admin.key(), 
            bump: bumps.marketplace, 
            treasure_bump: bumps.treasury, 
            name,
        });

        Ok(())
    }
}