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
        email: String,
        number_of_deposits: u64, total_amount_deposited: u64
    ) -> Result<()> {
        instructions::create_profile(ctx, name, email, number_of_deposits, total_amount_deposited)
    }

    // Join Pot
    pub fn user_join_pot(ctx: Context<JoinPot>) -> Result<()> {
        instructions::join_pot(ctx)
    }

    // Deposit into pot
    pub fn deposit(ctx:Context<DepositIntoPot>, amount:u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
}
