use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;

use constant::*;
use error::*;
use states::{TodoAccount, UserProfile};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_third {
    use super::*;

    pub fn initialize_user(ctx: Context<Initialize>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        // 여기서 key()는 퍼블릭 키를 의미합니다.
        // -> https://docs.rs/anchor-lang/0.18.0/anchor_lang/trait.Key.html

        user_profile.last_todo = 0;
        user_profile.todo_count = 0;

        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, _content: String) -> Result<()> {
        Ok(())
    }

    pub fn mark_todo(ctx: Context<MarkTodo>, todo_index: u8) -> Result<()> {
        Ok(())
    }

    pub fn remove_todo(ctx: Context<RemoveTodo>, todo_index: u8) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // Signer는 누가 해당 트랜잭션을 approve하는 지에 대한 정보 입니다.
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()], // 랜덤한 값을 만드는 방법입니다.
        // 초기 사용자를 암호화 하는데에 사용이 됩니다.
        bump, // seeds로 설정한 값이 있을시에만 동작을 하는 역할을 합니다.
        // 만약 seed가 없다면 바로 bump됩니다. : 에러가 발생합니다.
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>()
        // 코드의 사이즈를 의미합니다. 기본적으로 8의 값을 가지고 있고 이후 사용하는 구조체의 값을 더해 줍니다.
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    // Box는 rust가 가지고 있는 인메모리를 말합니다.
    // -> 사실 이 부분은 잘 이해가 되지 않아서 후에 또 알아보도록 하겠습니다.
}

#[derive(Accounts)]
pub struct AddTodo {}

#[derive(Accounts)]
pub struct MarkTodo {}

#[derive(Accounts)]
pub struct RemoveTodo {}

pub fn is_zero_account(account_info: &AccountInfo) -> bool {
    // 들어오는 주소값이 zero인지 체크하는 함수
    let account_data: &[u8] = &account_info.data.borrow();
    // AccountInfo의 data타입은 RefCel
    // 이중 borrow데이터가 의미하는 것은 wrap되어 있는 데이터를 가져오는 것을 의미한다.
    // -> https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html#method.borrow

    let len = account_data.len();

    for i in 0..len - 1 {
        if account_data[i] != 0 {
            return false;
        }
    }

    true
}
