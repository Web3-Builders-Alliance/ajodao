use crate::{state::pot::*, state::profile::*};
use anchor_lang::prelude::*;
use anchor_spl::{*, token::{Transfer, TokenAccount}};

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
    pub user_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositIntoPot<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        let cpi_account = Transfer {
            from: self.user.to_account_info(),
            to: self.pot.to_account_info(),
            authority: self.pot.to_account_info()
        };

        Ok(())
    }
}