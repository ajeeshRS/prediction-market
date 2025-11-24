use anchor_lang::{prelude::*,};
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount,Transfer};

use crate::{
    error::PredictionMarketErrors,
    market::{Market, WinningOutcome},
    MARKET_SEED,
};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
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

impl<'info> ClaimReward<'info> {
    pub fn claim(&mut self) -> Result<()> {
        require!(
            self.market.is_settled,
            PredictionMarketErrors::MarketNotSettled
        );

        let winning_outcome = self.market.winning_outcome.unwrap();

        let (winning_mint, user_ata) = match winning_outcome {
            WinningOutcome::OutcomeA => {
                (self.outcome_a_mint.to_account_info(), &self.user_a_outcome)
            }
            _ => (self.outcome_b_mint.to_account_info(), &self.user_b_outcome),
        };

        let amount = user_ata.amount;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: winning_mint,
            from: user_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)?;

        let cpi_accounts1 = Transfer {
            from: self.collateral_vault.to_account_info(),
            to: self.user_collateral_vault.to_account_info(),
            authority: self.market.to_account_info()
        };

        let maker_key = self.maker.key();
        let seeds = &[
            MARKET_SEED.as_bytes(),
            maker_key.as_ref(),
            &[self.market.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts1,
            signer_seeds,
        );

        transfer(ctx, amount)?;

        self.market.total_collateral_locked -= amount;

        Ok(())
    }
}
