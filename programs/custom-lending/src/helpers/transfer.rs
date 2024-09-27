use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, Mint, TransferChecked};

pub fn transfer_sol<'info>(program: AccountInfo<'info>, signer_seeds: &[&[&[u8]]], to: AccountInfo<'info>, from: AccountInfo<'info>, amount: u64) -> Result<()> {
    let accounts = Transfer {
        from,
        to,
    };
    let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
    transfer(ctx, amount)
}

pub fn transfer_spl<'info>(program: AccountInfo<'info>, signer_seeds: &[&[&[u8]]], mint: InterfaceAccount<'info, Mint>, to: AccountInfo<'info>, from: AccountInfo<'info>, authority: AccountInfo<'info>, amount: u64) -> Result<()> {
    let accounts = TransferChecked {
        from,
        to,
        mint: mint.to_account_info(),
        authority,
    };

    let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

    transfer_checked(ctx, amount, mint.decimals)
}

pub fn close_spl_account<'info>(program: AccountInfo<'info>, signer_seeds: &[&[&[u8]]], account: AccountInfo<'info>, destination: AccountInfo<'info>, authority: AccountInfo<'info>) -> Result<()> {
    let accounts = CloseAccount {
        account,
        destination,
        authority,
    };

    let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

    close_account(ctx)
}

pub fn accept_sol<'info>(program: AccountInfo<'info>, to: AccountInfo<'info>, from: AccountInfo<'info>, amount: u64) -> Result<()> {
    let accounts = Transfer {
        from,
        to,
    };
    let ctx = CpiContext::new(program, accounts);
    transfer(ctx, amount)
}

pub fn accept_spl<'info>(program: AccountInfo<'info>, mint: InterfaceAccount<'info, Mint>, to: AccountInfo<'info>, from: AccountInfo<'info>, authority: AccountInfo<'info>, amount: u64) -> Result<()> {
    let accounts = TransferChecked {
        from,
        to,
        mint: mint.to_account_info(),
        authority,
    };

    let ctx = CpiContext::new(program, accounts);

    transfer_checked(ctx, amount, mint.decimals)
}