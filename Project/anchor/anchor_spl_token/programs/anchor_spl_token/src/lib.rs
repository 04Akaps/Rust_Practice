use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub mod errors;

use errors::*;

declare_id!("Fi6agP72G3qHHidMNTsNpMrqv4DgqJGQr1iEDUrYuTiV");

#[program]
pub mod spl_token_program {
    use super::*;

    pub fn transfer(ctx: Context<TransferStruct>, amount: u64) -> Result<()> {
        let balance: u64 = ctx.accounts.sender_token.amount;
        msg!("starting tokens : {:?}", &balance);
        // 간단하게 sender의 associated account에 있는 token물량을 확인한다.

        require!(balance > amount, TodoError::NotEnoughBalanceFromSender);
        // 만약 가지고 있는 물량보다 적다면 error 발생

        token::transfer(ctx.accounts.transfer_ctx(), amount)?;

        ctx.accounts.sender_token.reload()?;

        msg!("remaining tokens  : {:?}", balance);

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct MintStruct<'info> {}

#[derive(Accounts)]
pub struct TransferStruct<'info> {
    pub sender: Signer<'info>,
    // sender의 wallet
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // sender의 associated Address

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,
    // receiver의 associated account
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> TransferStruct<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.sender_token.to_account_info(),
                to: self.receiver_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}
