use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        // constraint = maker_ata.owner == maker.key()
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub taker_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn make_offer(&self, amount: u64) -> Result<()> {
        //

        Ok(())
    }
}
