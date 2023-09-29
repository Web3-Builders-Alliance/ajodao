use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

use crate::state::pot::*;

pub fn create_pot(
    ctx: Context<CreatePot>,
    description: String,
    name: String,
    cycle: PotCycles,
    created_at: String,
    max_capacity: u8,
    contribution_amount: u64,
) -> Result<()> {
    ctx.accounts.pot.set_inner(Pot::new_pot(
        ctx.accounts.payer.key(),
        0,
        description,
        name,
        cycle,
        created_at,
        vec![],
        *ctx.bumps.get("vault").expect("Failed to get bump `vault`"),
        *ctx.bumps.get("pot").expect("Failed to get bump `pot`"),
        PotStatus::Open,
        max_capacity,
        contribution_amount,
        vec![],
    )?);
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePot<'info> {
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
    /// CHECK: This is fine
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
