use anchor_lang::prelude::*;

#[error_code]
pub enum PredictionMarketErrors {
    #[msg("Amount cannot be zero")]
    AmountCannotBeZero,
    #[msg("Market already settled")]
    MarketAlreadySettled,
    #[msg("Market not settled")]
    MarketNotSettled,
    #[msg("Market expired")]
    MarketExpired,
    #[msg("Invalid outcome")]
    InvalidOutcome,
}
