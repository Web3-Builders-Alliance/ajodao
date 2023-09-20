use anchor_lang::prelude::*;

use crate::profile::*;

#[account]
pub struct Pot {
    pub owner: Pubkey,
    pub total_amount: u64,
    pub description: String,
    pub name: String,
    pub cycle: PotCycles,
    pub created_at: String,
    pub members: Vec<Profile>,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Pot {
    pub const POT_PREFIX: &'static str = "pot";

    // This creates a new pot with the following details
    pub fn new_pot(
        owner: Pubkey,
        total_amount: u64,
        description: String,
        name: String,
        cycle: PotCycles,
        created_at: String,
        members: Vec<Profile>,
        vault_bump: u8,
        state_bump: u8, 
    ) -> Result<Self> {
        Ok(Self {
            owner,
            total_amount,
            description,
            name,
            cycle,
            created_at,
            members,
            vault_bump,
            state_bump,
        })
    }

    // Join a pot
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PotCycles {
    Daily,
    Weekly,
    Monthly,
}
