use anchor_lang::prelude::*;

declare_id!("5DmyuB6gckG5sqxMEeWqUg3SifmmfPrBoYpqVQJvtsmk");

#[program]
pub mod solana_loan_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
