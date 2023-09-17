use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::pot::*;

pub fn create_pot(
    ctx: Context<CreatePot>,
    description: String,
    name: String,
    cycle: PotCycles,
    created_at: String,
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
    )?);
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    owner: Pubkey,
    description: String,
    name: String,
    cycle: PotCycles,
)]
pub struct CreatePot<'info> {
    #[account(
        init,
        space = size_of::<Pot>(),
        payer = payer,
        seeds = [
            b"pot",
            owner.key().as_ref(),
            payer.key().as_ref(),
            description.as_bytes().as_ref()
        ],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [
            b"vault",
            pot.key().as_ref()
        ],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
