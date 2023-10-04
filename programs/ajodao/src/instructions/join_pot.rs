use crate::state::{errors::*, pot::*, profile::*};
use anchor_lang::prelude::*;
// use std::mem::size_of;

// pub fn join_pot(ctx: Context<JoinPot>) -> Result<()> {
//     // require!(profile.address, );
//     if ctx.accounts.profile.address != *ctx.accounts.payer.key {
//         return Err(Errors::UserProfileNotFound.into());
//     }

// Checks for user already in pot
// if ctx.accounts.pot.members.contains(&ctx.accounts.profile) {
//     return Err(Errors::UserAlreadyInPot.into());
// }

//     if ctx.accounts.pot.members.len() == ctx.accounts.pot.max_capacity as usize {
//         // Update pot status here and emit an event
//         ctx.accounts.pot.pot_status = PotStatus::InProgress;
//         // Todo: Emit an event.
//         return Err(Errors::MaximumCapacityReached.into());
//     }

//     // ctx.accounts.pot.members.push(ctx.accounts.profile.to_account_info().data.clone())
//     let binding = ctx.accounts.profile.to_account_info();
//     let profile_data = binding.data.borrow_mut();
//     let profile = UserProfile::try_from_slice(&profile_data)?;
//     let _ = ctx.accounts.pot.join_pot(profile);

//     // Todo: Emit an event here of the user that joined.

//     Ok(())
// }

#[derive(Accounts)]
pub struct JoinPot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"pot", pot.owner.key().as_ref()],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}
