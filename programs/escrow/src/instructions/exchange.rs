use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ close_account, CloseAccount },
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::state::escrow::EscrowState;

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = 8 + EscrowState::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowState>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = escrow
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
       associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
       associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub taker_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Exchange<'info> {
    pub fn take_offer(&mut self) -> Result<()> {
        // check if it has no balance  up to required;

        //

        let binding = self.escrow.maker;
        let binding1 = self.escrow.seed.to_le_bytes();

        let seeds = &[b"escrow", binding.as_ref(), binding1.as_ref(), &[self.escrow.bump]];

        let signer_seeds = &[&seeds[..]];

        // send maker the required amount

        let cpi_context_for_transfer_to_maker = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.vault.to_account_info(),
                to: self.taker_ata.to_account_info(),
                authority: self.maker.to_account_info(),
                mint: self.maker_mint.to_account_info(),
            }
        );

        transfer_checked(
            cpi_context_for_transfer_to_maker,
            self.escrow.wants,
            self.taker_mint.decimals
        )?;

        // give taker the exchange amount;

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.vault.to_account_info(),
                to: self.taker_ata.to_account_info(),
                authority: self.maker.to_account_info(),
                mint: self.maker_mint.to_account_info(),
            },
            signer_seeds
        );

        transfer_checked(cpi_context, self.escrow.deposited, self.maker_mint.decimals)?;

        //  close the vault account;

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.escrow.to_account_info(),
            },
            signer_seeds
        );

        close_account(ctx)?;

        Ok(())
    }
}
