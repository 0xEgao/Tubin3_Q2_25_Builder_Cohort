use crate::error::ErrorCode;
use crate::Ecomint;
use crate::Marketplace;
use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

//send usdc/sol from buyer to organisation account

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut)]
    pub taker: Signer<'info>, //buyer account

    #[account(mut, address=eco_mint.maker)]
    pub maker: SystemAccount<'info>, //seller-organisation account

    #[account(mut,address=marketplace.admin)]
    pub admin: SystemAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, //USDC mint
    #[account(
        mut,
        has_one=maker,
        seeds=[b"ecomint",eco_mint.maker.key().as_ref()],
        bump
    )]
    pub eco_mint: Account<'info, Ecomint>,

    #[account(
        seeds=[b"marketplace",eco_mint.maker.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=taker,
        associated_token::authority=admin
    )]
    pub admin_usdc: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=taker,
        associated_token::authority=taker,
    )]
    pub taker_usdc: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint,
        associated_token::authority=maker,
    )]
    pub maker_usdc: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Send<'info> {
    pub fn send_usdc(&mut self, amount: u16) -> Result<()> {
        //calculate usdc
        let total_usdc = self
            .eco_mint
            .offset_value
            .checked_mul(amount)
            .ok_or(ErrorCode::CalculationOverflow)?;

        //apply marketplace fee
        let fee = total_usdc
            .checked_mul(self.marketplace.fee)
            .ok_or(ErrorCode::CalculationOverflow)?
            .checked_div(100)
            .ok_or(ErrorCode::CalculationOverflow)?;

        let seller_amount = total_usdc
            .checked_sub(fee)
            .ok_or(ErrorCode::CalculationUnderflow)?;

        //Transfer usdc from buyer to org account
        let transfer = TransferChecked {
            from: self.taker_usdc.to_account_info(),
            to: self.maker_usdc.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer);

        transfer_checked(
            cpi_ctx,
            seller_amount.try_into().unwrap(),
            self.mint.decimals,
        )?;
        Ok(())
    }

    pub fn send_fee(&mut self, amount: u16) -> Result<()> {
        //total usdc fee
        let total_usdc = self
            .eco_mint
            .offset_value
            .checked_mul(amount)
            .ok_or(ErrorCode::CalculationOverflow)?;

        //marketplace fee
        let fee = total_usdc
            .checked_mul(self.marketplace.fee)
            .ok_or(ErrorCode::CalculationOverflow)?
            .checked_div(100)
            .ok_or(ErrorCode::CalculationOverflow)?;

        //transfer fee to Marketplace owner i.e the admin

        let fee_transfer = TransferChecked {
            from: self.taker_usdc.to_account_info(),
            to: self.admin_usdc.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let fee_ctx = CpiContext::new(self.token_program.to_account_info(), fee_transfer);

        transfer_checked(fee_ctx, fee.try_into().unwrap(), self.mint.decimals)?;

        Ok(())
    }
}
