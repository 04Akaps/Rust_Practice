use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;

pub mod constant;
pub mod error;
pub mod states;
pub mod util;

use constant::*;
use error::*;
use states::{StateAccount, UserAccount};
use util::bump;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_fourth {
    use super::*;

    pub fn init_state_account(ctx: Context<InitStateAccount>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();
        state.video_count = 0;

        Ok(())
    }

    pub fn create_user(
        ctx: Context<CreateUser>,
        _username: String,
        _user_image_url: String,
    ) -> Result<()> {
        if _username.trim().is_empty() || _user_image_url.trim().is_empty() {
            // 인자값이 빈값이면 에러를 리턴
            return Err(Errors::CannotCreateUser.into());
        }

        let user = &mut ctx.accounts.user;

        user.user_wallet_address = ctx.accounts.authority.key();

        user.user_name = _username;
        user.user_profile_image_url = _user_image_url;

        msg!("User Added");
        sol_log_compute_units();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStateAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [b"state".as_ref(), authority.key().as_ref()],
        // as_ref와 borrow는 유사하게 동작하지만 살짝 다릅니다.
        // as_ref는 변환의 목적, borrow는 빌림의 목적에 의미를 둡니다.
        // 예를들면 String은 borrow를 통해서 리터럴 타입 &str을 얻을 수 있게 됩니다.
        // 이에 반해 as_ref는 어떤 타입을 다른 타입으로 변환하여 쓰기 위한 곳에 사용이 됩니다.
        // https://doc.rust-lang.org/std/convert/trait.AsRef.html
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<StateAccount>()
    )]
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"state".as_ref(), authority.key().as_ref()],
        // b''은 바이트 리터럴을 의미합니다.
        bump,
        has_one = authority,
    )]
    pub state: Account<'info, StateAccount>,

    #[account (
        init,
        seeds = [b"user".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserAccount>()
    )]
    pub user: Account<'info, UserAccount>,
}
