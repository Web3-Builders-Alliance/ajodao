use anchor_lang::prelude::*;

#[account]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub number_of_deposits: u64,
    pub total_amount_deposited: u64
}

impl Profile {
    pub fn new_profile(name: String, email: String, number_of_deposits: u64, total_amount_deposited: u64) -> Result<Self> {
        Ok(Self { name, email, number_of_deposits, total_amount_deposited })
    }
}
