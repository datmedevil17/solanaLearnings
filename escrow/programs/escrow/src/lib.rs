#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("GSrB3A2d5kBrqmuFtAxTj5r3okiH5BJ6EPAVWyRHrCVm");

pub mod state;
pub mod instructions;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}




// 
