use crate::state::{errors::*, pot::*, profile::*};
use anchor_lang::prelude::*;
// use std::mem::size_of;

pub fn join_pot(ctx: Context<JoinPot>, _name: String, _creator: Pubkey) -> Result<()> {
    if ctx.accounts.members.name == " " {
        return Err(Errors::UserProfileNotFound.into())
    }

    // Check whether number of pot is equal to the max_capacity
    // If it is, return error
    if ctx.accounts.pot.max_capacity == ctx.accounts.pot.num_of_members_joined {
        return Err(Errors::MaximumCapacityReached.into());
    }

    // Check if user is already in the Pot
    if ctx.accounts.pot.members.contains(&ctx.accounts.payer.key()) {
        return Err(Errors::UserAlreadyInPot.into())
    }

    ctx.accounts.pot.members.push(ctx.accounts.payer.key());

    // Increment number of members in the pot
    ctx.accounts.pot.num_of_members_joined += 1;

    // Todo: Emit an event here of the user that joined.

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, creator: Pubkey)]
pub struct JoinPot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [name.as_bytes(), creator.key().as_ref()],
        bump,
        realloc = 8 + 24 + 12 + 100 + 20 + 10 + 15 + 10 + 15 + 10 + 10 + 10 + 12 + 200 + 1,
        realloc::payer = payer,
        realloc::zero = true
    )]
    pub pot: Account<'info, Pot>,
    #[account(
        seeds = [
            b"profile",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub members: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}
