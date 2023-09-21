use anchor_lang::prelude::*;
use crate::{state::pot::*, state::profile::*};
use std::mem::size_of;

pub fn join_pot(_ctx: Context<JoinPot>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct JoinPot<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"pot", pot.owner.key().as_ref()],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(
        init,
        seeds = [b"user", pot.owner.key().as_ref(), payer.key().as_ref()],
        bump,
        space = size_of::<Profile>() + 1,
        payer = payer
    )]
    pub user: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}