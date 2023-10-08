use anchor_lang::prelude::*;

#[account]
#[derive(PartialEq)]
pub struct UserProfile {
    pub number_of_deposits: u64,
    pub total_amount_deposited: u64,
    pub name: String,
    pub email: String,
}

impl Space for UserProfile {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 4 + 30 + 4 + 30;
}

impl UserProfile {
    pub fn new_profile(
        name: String,
        email: String,
        number_of_deposits: u64,
        total_amount_deposited: u64,
    ) -> Result<Self> {
        Ok(Self {
            number_of_deposits,
            total_amount_deposited,
            name,
            email,
        })
    }
}
