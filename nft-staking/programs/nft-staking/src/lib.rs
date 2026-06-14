#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod contexts;
mod states;
use contexts::*;
mod error;

declare_id!("5bNSoPr5KQwr3pLpbP3pq64RNSe4tuKenAuJjFezNTqY");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
