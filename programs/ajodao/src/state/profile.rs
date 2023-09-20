use anchor_lang::prelude::*;

#[account]
pub struct Profile {
    pub name: String,
    pub email: String,
}

impl Profile {
    pub fn new_profile(name: String, email: String) -> Result<Self> {
        Ok(Self { name, email })
    }
}
