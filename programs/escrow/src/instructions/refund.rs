use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint,
        TokenAccount,
        TransferChecked,
        TokenInterface,
        transfer_checked,
        close_account,
        CloseAccount,
    },
};

use crate::state::escrow::EscrowState;

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Refund<'info> {
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

    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub taker_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Refund<'info> {
    pub fn cancel_offer(&mut self, deposit_amount: u64, amount_wanted: u64, seed: u64) -> Result<()> {
        //
        self.escrow.set_inner(EscrowState {
            maker: self.maker.key(),
            deposited: deposit_amount,
            wants: amount_wanted,
            bump: self.escrow.bump,
            taker_mint: self.taker_mint.key(),
            maker_mint: self.maker_mint.key(),
            seed: seed,
        });

        let binding1 = self.escrow.maker.key();
        let binding2 = self.escrow.seed.to_le_bytes();

        let seeds = [b"escrow", binding1.as_ref(), binding2.as_ref(), &[self.escrow.bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.vault.to_account_info(),
                to: self.maker_ata.to_account_info(),
                authority: self.maker.to_account_info(),
                mint: self.maker_mint.to_account_info(),
            },
            signer_seeds
        );

        transfer_checked(cpi_context, deposit_amount, self.maker_mint.decimals)?;

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
