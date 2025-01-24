use anchor_lang::prelude::*;

declare_id!("6A2iXHYqQXdZq428g3fgrfE4DgvSF73ACKav2U6joWp8");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
