use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("6A2iXHYqQXdZq428g3fgrfE4DgvSF73ACKav2U6joWp8");

#[program]
pub mod escrow {
    use crate::instructions::Refund;

    use super::*;

    pub fn make_offer(ctx: Context<Make>, amount: u64, seed: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn take_offer(ctx: Context<Exchange>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn cancel_offer(ctx: Context<Refund>, seed: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
