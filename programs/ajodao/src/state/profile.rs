use anchor_lang::prelude::*;

#[account]
#[derive(PartialEq)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub number_of_deposits: u64,
    pub total_amount_deposited: u64,
    pub address: Pubkey
}

impl Profile {
    pub fn new_profile(name: String, email: String, number_of_deposits: u64, total_amount_deposited: u64, address: Pubkey) -> Result<Self> {
        Ok(Self { name, email, number_of_deposits, total_amount_deposited, address })
    }
}
