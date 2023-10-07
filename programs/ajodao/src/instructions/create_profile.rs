use anchor_lang::prelude::*;

use crate::state::profile::*;

pub fn create_profile(ctx: Context<CreateProfile>, name: String, email: String) -> Result<()> {
    require_gt!(31, name.len());
    require_gt!(31, email.len());
    let profile = UserProfile::new_profile(name, email, 0, 0, vec![])?;
    ctx.accounts.profile.set_inner(profile);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = UserProfile::INIT_SPACE,
        seeds = [b"profile", payer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}
