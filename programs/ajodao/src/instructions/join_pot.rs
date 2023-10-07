use crate::state::{errors::*, pot::*, profile::*};
use anchor_lang::prelude::*;
// use std::mem::size_of;

pub fn join_pot(ctx: Context<JoinPot>, _name: String, _creator: Pubkey) -> Result<()> {
    // require!(profile.address, );
    // if ctx.accounts.profile.address != *ctx.accounts.payer.key {
    //     return Err(Errors::UserProfileNotFound.into());
    // }
    if ctx.accounts.profile.name == " " {
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

    // if ctx.accounts.pot.members.contains(&ctx.accounts.profile) {
    //     return Err(Errors::UserAlreadyInPot.into());
    // }

    // if ctx.accounts.pot.members.len() == ctx.accounts.pot.max_capacity as usize {
    //     // Update pot status here and emit an event
    //     ctx.accounts.pot.pot_status = PotStatus::InProgress;
    //     // Todo: Emit an event.
    //     return Err(Errors::MaximumCapacityReached.into());
    // }

    // ctx.accounts.pot.members.push(ctx.accounts.profile.to_account_info().data.clone())
    // let binding = ctx.accounts.profile.to_account_info();
    // let profile_data = binding.data.borrow_mut();
    // let profile = UserProfile::try_from_slice(&profile_data)?;
    // let _ = ctx.accounts.pot.join_pot(profile);

    // Todo: Emit an event here of the user that joined.

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, creator: Pubkey)]
pub struct JoinPot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [name.as_bytes(), creator.key().as_ref()],
        bump
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
    #[account(mut)]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}
