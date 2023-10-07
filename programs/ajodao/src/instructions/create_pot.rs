use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{state::pot::*, UserProfile, Errors};

pub fn create_pot(
    ctx: Context<CreatePot>,
    description: String,
    name: String,
    cycle: PotCycles,
    created_at: String,
    max_capacity: u8,
    contribution_amount: u64,
) -> Result<()> {
    // Check for created profile
    if ctx.accounts.members.name == "" {
        return Err(Errors::UserProfileNotFound.into())
    }

    ctx.accounts.pot.set_inner(Pot::new_pot(
        ctx.accounts.payer.key(),
        0,
        description,
        name,
        cycle,
        created_at,
        // vec![],
        *ctx.bumps.get("vault").expect("Failed to get bump `vault`"),
        PotStatus::Open,
        max_capacity,
        contribution_amount,
        0, // vec![],
        vec![]
    )?);

    // Add user to members vec![]
    ctx.accounts.pot.members.push(ctx.accounts.payer.key());

    // Add public key of pot to profile pots created
    ctx.accounts.members.pots_created.push(ctx.accounts.pot.key());

    // Increments number of members that have joiined 
    // since we're generating a member PDA for the creator
    ctx.accounts.pot.num_of_members_joined += 1;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    description: String,
    name: String,
    cycle: PotCycles,
    created_at: String,
    max_capacity: u8,
    contribution_amount: u64
)]
pub struct CreatePot<'info> {
    #[account(
        init,
        space = 8 + 24 + 12 + 100 + 20 + 10 + 15 + 10 + 15 + 10 + 10 + 10 + 12 + 200 + 1,
        payer = payer,
        seeds = [
            name.as_bytes(),
            payer.key().as_ref(),
        ],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [
            b"profile",
            payer.key().as_ref()
        ],
        bump
    )]
    pub members: Account<'info, UserProfile>,
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
