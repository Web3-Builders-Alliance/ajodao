use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instruction::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ajodao {
    use super::*;
}
