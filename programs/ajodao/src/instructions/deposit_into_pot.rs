use crate::{state::pot::*, state::profile::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, TokenAccount, Token, transfer, Mint};

pub fn deposit_into_pot() -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DepositIntoPot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"pot", pot.owner.key().as_ref()],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(
        seeds = [b"user", pot.owner.key().as_ref(), payer.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, Profile>,
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [
            b"vault",
            pot.key().as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = auth
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub user_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositIntoPot<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        let cpi_account = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi = CpiContext::new(self.token_program.to_account_info(), cpi_account);

        transfer(cpi, amount)

        // Ok(())
    }
}