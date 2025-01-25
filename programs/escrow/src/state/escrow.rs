use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub maker: Pubkey,
    pub taker_mint: Pubkey,
    pub maker_mint: Pubkey,
    pub deposited: u64,
    pub wants: u64,
    pub seed: u64,
    pub bump: u8,
}
