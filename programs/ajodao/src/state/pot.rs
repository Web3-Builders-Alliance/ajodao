use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

// use crate::errors::*;
// use crate::profile::*;

#[account]
pub struct Pot {
    pub owner: Pubkey,
    pub total_amount: u64,
    pub description: String,
    pub name: String,
    pub cycle: PotCycles,
    pub created_at: String,
    pub vault_bump: u8,
    pub pot_status: PotStatus,
    pub max_capacity: u8,
    pub contribution_amount: u64,
    pub num_of_members_joined: u8,
    pub members: Vec<Pubkey>,
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
        vault_bump: u8,
        pot_status: PotStatus,
        max_capacity: u8,
        contribution_amount: u64,
        num_of_members_joined: u8,
        members: Vec<Pubkey>,
    ) -> Result<Self> {
        Ok(Self {
            owner,
            total_amount,
            description,
            name,
            cycle,
            created_at,
            vault_bump,
            pot_status,
            max_capacity,
            contribution_amount,
            num_of_members_joined,
            members,
        })
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PotCycles {
    Daily,
    Weekly,
    Monthly,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PotStatus {
    Open,
    Closed,
    InProgress,
}

#[account()]
pub struct PotBalance {
    pot: Pubkey,
    amount: u64,
}
