pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
use crate::market::WinningOutcome;

declare_id!("5DhpwtZVzQ9Qva6PCeg9vLzkkdC19qPF5SnoTBRwdCtR");


#[program]
pub mod prediction_market {

    use super::*;

    pub fn init_market(ctx: Context<InitializeMarket>, market_id: u32) -> Result<()> {
        ctx.accounts.init(market_id, ctx.bumps)
    }

    pub fn split_tokens(ctx: Context<SplitTokens>, amount: u64) -> Result<()> {
        ctx.accounts.split(amount)
    }

    pub fn merge_tokens(ctx: Context<MergeTokens>) -> Result<()> {
        ctx.accounts.merge()
    }

    pub fn settle_market(ctx: Context<SettleMarket>,winning_outcome:WinningOutcome) -> Result<()> {
        ctx.accounts.settle(winning_outcome)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        ctx.accounts.claim()
    }
}
