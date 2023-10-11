use crate::{state::pot::*, state::profile::*, Errors};
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
// use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(amount: u64, name: String, creator: Pubkey)]
pub struct DepositIntoPot<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), creator.key().as_ref()],
        bump,
        realloc = 8 + 24 + 12 + 100 + 20 + 10 + 15 + 10 + 15 + 10 + 10 + 10 + 12 + 200 + 1,
        realloc::payer = payer,
        realloc::zero = true
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [
            b"profile",
            payer.key().as_ref()
        ],
        bump
    )]
    pub members: Account<'info, UserProfile>,
    // #[account(
    //     seeds = [b"auth", pot.key().as_ref()],
    //     bump
    // )]
    /// CHECK: This is fine
    // pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [
            b"vault",
            pot.key().as_ref()
        ],
        bump,
        // token::mint = token_mint,
        // token::authority = auth
    )]
    pub vault: SystemAccount<'info>,
    // pub token_mint: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     associated_token::mint = token_mint,
    //     associated_token::authority = payer
    // )]
    // pub user_ata: Account<'info, TokenAccount>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
} 

impl<'info> DepositIntoPot<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        if amount != self.pot.contribution_amount {
            return Err(Errors::ContributionAmountDoesNotEqualPotContributionAmount.into());
        }

        let cpi_account = Transfer {
            from: self.payer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), cpi_account);
        self.pot.total_amount += amount;

        transfer(cpi, amount)

        // Ok(())
    }
}
