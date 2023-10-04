use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("Er6BnuDMkq71j2v7TJZRkNPtVwFGJ31iCiGbuYcHLGKk");

#[program]
pub mod ajodao {
    use super::*;

    pub fn create_new_pot(
        ctx: Context<CreatePot>,
        description: String,
        name: String,
        cycle: PotCycles,
        created_at: String,
        max_capacity: u8,
        contribution_amount: u64,
    ) -> Result<()> {
        instructions::create_pot(
            ctx,
            description,
            name,
            cycle,
            created_at,
            max_capacity,
            contribution_amount,
        )
    }

    // Create Profile
    pub fn create_new_profile(
        ctx: Context<CreateProfile>,
        name: String,
        email: String,
    ) -> Result<()> {
        instructions::create_profile(ctx, name, email)
    }

    // Join Pot
    // pub fn user_join_pot(ctx: Context<JoinPot>) -> Result<()> {
    //     instructions::join_pot(ctx)
    // }

    // Deposit into pot
    pub fn deposit(ctx: Context<DepositIntoPot>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // Update Pot Open Status
    // pub fn update_pot_status(ctx: Context<UpdatePotOpenStatus>, status: bool) -> Result<()> {
    //     ctx.accounts.update_pot_open_status(status)
    // }
}
