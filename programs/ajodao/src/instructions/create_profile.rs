use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::profile::*;

pub fn create_profile(ctx: Context<CreateProfile>, name: String, email: String) -> Result<()> {
    ctx.accounts.profile.set_inner(UserProfile::new_profile(
        name,
        email,
        0,
        0,
        ctx.accounts.payer.key(),
    )?);
    Ok(())
}

#[derive(Accounts, AnchorDeserialize, AnchorSerialize)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = size_of::<UserProfile>() + 8,
        seeds = [b"profile", payer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}
