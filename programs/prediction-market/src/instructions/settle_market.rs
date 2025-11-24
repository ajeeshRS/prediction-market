use anchor_lang::{prelude::*, system_program::TransferBumps};
use anchor_spl::token::{
    set_authority, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token, TokenAccount,
};

use crate::{
    error::PredictionMarketErrors,
    market::{Market, WinningOutcome},
    MARKET_SEED,
};

#[derive(Accounts)]
pub struct SettleMarket<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds =[MARKET_SEED.as_bytes(),maker.key().as_ref()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

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

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SettleMarket<'info> {
    pub fn settle(&mut self, winning_outcome: WinningOutcome) -> Result<()> {
        require!(
            !self.market.is_settled,
            PredictionMarketErrors::MarketAlreadySettled
        );

        require!(
            matches!(
                winning_outcome,
                WinningOutcome::OutcomeA | WinningOutcome::OutcomeB
            ),
            PredictionMarketErrors::InvalidOutcome
        );

        self.market.is_settled = true;

        self.market.winning_outcome = Some(winning_outcome);

        self.revoke_mint_authority(true)?;
        self.revoke_mint_authority(false)?;

        Ok(())
    }

    pub fn revoke_mint_authority(&mut self, is_a: bool) -> Result<()> {
        let mint = match is_a {
            true => self.outcome_a_mint.to_account_info(),
            false => self.outcome_b_mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = SetAuthority {
            account_or_mint: mint,
            current_authority: self.market.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seeds = &[
            MARKET_SEED.as_bytes(),
            maker_key.as_ref(),
            &[self.market.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;

        Ok(())
    }
}
