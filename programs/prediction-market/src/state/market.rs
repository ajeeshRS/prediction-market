use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum WinningOutcome {
    OutcomeA,
    OutcomeB,
    None,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub maker: Pubkey,
    pub market_id: u32,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub outcome_a_mint: Pubkey,
    pub outcome_b_mint: Pubkey,
    pub is_settled: bool,
    pub deadline: i64,
    pub winning_outcome: Option<WinningOutcome>,
    pub total_collateral_locked: u64,
    pub bump: u8,
}
