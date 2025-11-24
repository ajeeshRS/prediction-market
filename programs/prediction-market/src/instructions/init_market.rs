use crate::{
    market::{Market},
    MARKET_SEED, OUTCOME_A_SEED, OUTCOME_B_SEED, VAULT_SEED,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
// #[instruction(market_id:u32)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = Market::INIT_SPACE + 8,
        seeds =[MARKET_SEED.as_bytes(),maker.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub collateral_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = maker,
        token::mint= collateral_mint,
        token::authority = market,
        seeds =[VAULT_SEED.as_bytes(),market.key().as_ref()],
        bump
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        mint::decimals= 6,
        mint::authority = market,
        seeds =[OUTCOME_A_SEED.as_bytes(),market.key().as_ref()],
        bump
    )]
    pub outcome_a_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = maker,
        mint::decimals= 6,
        mint::authority = market,
        seeds =[OUTCOME_B_SEED.as_bytes(),market.key().as_ref()],
        bump
    )]
    pub outcome_b_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarket<'info> {
    pub fn init(&mut self, market_id: u32, bumps: InitializeMarketBumps) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        let deadline = now + 5 * 24 * 60 * 60;

        self.market.set_inner(Market {
            maker: self.maker.key(),
            market_id,
            collateral_mint: self.collateral_mint.key(),
            collateral_vault: self.collateral_vault.key(),
            outcome_a_mint: self.outcome_a_mint.key(),
            outcome_b_mint: self.outcome_b_mint.key(),
            is_settled: false,
            deadline,
            winning_outcome: None,
            total_collateral_locked: 0,
            bump: bumps.market,
        });

        msg!("Market init success: {}", self.market.key());
        Ok(())
    }
}
