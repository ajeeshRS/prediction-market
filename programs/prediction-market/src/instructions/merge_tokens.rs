use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer};

use crate::{error::PredictionMarketErrors, market::Market, MARKET_SEED};

#[derive(Accounts)]
pub struct MergeTokens<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds =[MARKET_SEED.as_bytes(),maker.key().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    pub collateral_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = collateral_vault.key() == market.collateral_vault
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_collateral_vault.mint == market.collateral_mint,
        constraint = user_collateral_vault.owner == user.key()
    )]
    pub user_collateral_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = outcome_a_mint.key() == market.outcome_a_mint
    )]
    pub outcome_a_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = outcome_b_mint.key() == market.outcome_b_mint
    )]
    pub outcome_b_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_a_outcome.mint == market.outcome_a_mint,
    )]
    pub user_a_outcome: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_b_outcome.mint == market.outcome_b_mint,
    )]
    pub user_b_outcome: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> MergeTokens<'info> {
    pub fn merge(&mut self) -> Result<()> {
        require!(
            !self.market.is_settled,
            PredictionMarketErrors::MarketAlreadySettled
        );

        require!(
            Clock::get()?.unix_timestamp < self.market.deadline,
            PredictionMarketErrors::MarketExpired
        );

        let a_bal = self.user_a_outcome.amount;
        let b_bal = self.user_b_outcome.amount;

        let amount = a_bal.min(b_bal);

        require!(amount > 0, PredictionMarketErrors::AmountCannotBeZero);

        // burning the outcome tokens
        self.burn_outcome_token(true, amount)?;
        self.burn_outcome_token(false, amount)?;

        // transferring the collateral amount from vault to user
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.collateral_vault.to_account_info(),
            to: self.user_collateral_vault.to_account_info(),
            authority: self.market.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seeds = &[
            MARKET_SEED.as_bytes(),
            maker_key.as_ref(),
            &[self.market.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer(cpi_ctx, amount)?;

        self.market.total_collateral_locked -= amount;
        Ok(())
    }

    pub fn burn_outcome_token(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (mint, from) = match is_a {
            true => (
                self.outcome_a_mint.to_account_info(),
                self.user_b_outcome.to_account_info(),
            ),
            false => (
                self.outcome_b_mint.to_account_info(),
                self.user_b_outcome.to_account_info(),
            ),
        };

        let cpi_accounts = Burn {
            mint,
            from,
            authority: self.user.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seeds = &[
            MARKET_SEED.as_bytes(),
            maker_key.as_ref(),
            &[self.market.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        burn(cpi_ctx, amount)
    }
}
