use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ajodao {
    use super::*;

    pub fn create_new_pot(
        ctx: Context<CreatePot>,
        description: String,
        name: String,
        cycle: PotCycles,
        created_at: String,
    ) -> Result<()> {
        instructions::create_pot(ctx, description, name, cycle, created_at)
    }

    // Create Profile
    pub fn create_new_profile(
        ctx: Context<CreateProfile>,
        name: String,
        email: String
    ) -> Result<()> {
        instructions::create_profile(ctx, name, email)
    }
}
