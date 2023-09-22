use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use std::mem::size_of;

use crate::state::pot::*;

#[derive(Accounts)]
pub struct UpdatePotOpenStatus<'info> {
    #[account(
        init,
        space = size_of::<Pot>(),
        payer = payer,
        seeds = [
            b"pot",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"auth", pot.key().as_ref()],
        bump
    )]
    /// This is fine
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdatePotOpenStatus<'info> {
    pub fn update_pot_open_status(self: &mut Self, status: bool) -> Result<()> {
        self.pot.is_open = status;
        Ok(())
    }
}