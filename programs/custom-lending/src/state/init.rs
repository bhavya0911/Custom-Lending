use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Init {
    pub admin: Pubkey,
    pub bump: u8,
    pub treasure_bump: u8,
    pub name: String,
}

impl Space for Init {
    const INIT_SPACE: usize = ANCHOR_DISC + 2 * U8_L + 1 * PUBKEY_L + STRING_L * 1;
}