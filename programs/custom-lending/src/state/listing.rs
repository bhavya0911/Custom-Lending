use anchor_lang::prelude::*;

use crate::constants::{ANCHOR_DISC, BOOL_L, I64_L, PUBKEY_L, U64_L, U8_L};

#[account]
pub struct Listing {
    pub bump: u8,
    pub vault_bump: u8,
    pub seed: u64,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub listing_type: bool,
    pub collateral_token: Pubkey,
    pub collateral_amount: u64,
    pub loan_token: Pubkey,
    pub loan_amount: u64,
    pub creation_timestamp: i64,
    pub active: bool,
    pub expiry: i64,
    pub repay_time: i64,
}

impl Space for Listing {
    const INIT_SPACE: usize = ANCHOR_DISC + U8_L * 2 + U64_L * 3 + I64_L * 3 + PUBKEY_L * 4 + BOOL_L * 2;
}