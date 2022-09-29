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

        let user_profile = &mut ctx.accounts.user_profile;
        let todo_account = &mut ctx.accounts.user_todo;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.index = user_profile.last_todo;
        todo_account.content = _content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        // checked_add는 일반적인 더하기와 같습니다.
        // 하지만 overFlow를 방지하는 역할을 합니다. Solidity에서는 SafeMath랑 같은 역할을 하는 거라고 할 수 있습니다.
        // https://docs.rs/num/latest/num/trait.CheckedAdd.html
        Ok(())
    }

    pub fn mark_todo(ctx: Context<MarkTodo>, todo_index: u8) -> Result<()> {
        let todo_account = &mut ctx.accounts.user_todo;

        require!(!todo_account.marked, TodoError::AlreadyMarked);
        // require를 통해서 해당 값을 검정을 합니다.
        // 이전에 address즉 자신의 Todo인지를 체크 안하는 이유는 어차피 MarkTodo구조체에서 #[]값의 bump를 통해서 검증이 되기 떄문에 따로 Owner에 대한 검증이 필요 없기 떄문입니다.

        todo_account.marked = true;
        Ok(())
    }

    pub fn remove_todo(ctx: Context<RemoveTodo>, todo_index: u8) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let todo_account = &mut ctx.accounts.user_todo;

        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

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

    // 수정
    // https://doc.rust-lang.org/std/boxed/struct.Box.html
    // 힘 메모리를 가르키는 포인터 타입이라고 합니다.
    // Box를 사용하면 절대 할당된 메모리 이상으로 사용이 안되는 특징이 있습니다.
    // https://doc.rust-lang.org/std/boxed/index.html
}

#[derive(Accounts)]
pub struct AddTodo<'info> {
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump, 
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    // 수정해야 할 부분이 있어서 수정이 되었습니다.
    // has_one을 통해서 저희는 데이터를 가져 옵니다.
    // 이전에 init할 떄에는 그냥 단순히 init 및 payer를 지정하였지만
    // init을 하게 되면 데이터가 존재하게 되기 떄문에 이후 has_one 프로퍼티를 입력해서 데이터를 끌고오면 됩니다.
    // 만약 여기가 init 및 payer로 구성이 된다면 계속해서 새로운 address가 만들어 지게 될 것입니다.
    // https://docs.rs/anchor-lang/0.11.0/anchor_lang/derive.Accounts.html

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoAccount>()
    )]
    pub user_todo : Box<Account<'info, TodoAccount>>,
    // 여기는 새로운 todo를 더해주는 곳이기 떄문에 init을 활용합니다.
}

#[derive(Accounts)]
pub struct MarkTodo<'info> {
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_todo : Box<Account<'info, TodoAccount>>,

    // MarkTodo구조체는 어차피 TodoAccount구조체에서 marked값만 바꾸어 주기 떄문에 추가적인 값을 작성하지 않아도 됩니다.
}

#[derive(Accounts)]
pub struct RemoveTodo<'info> {
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump, 
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>()
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
        close = authority,
        //삭제하는 데이터에는 close를 사용해 줍니다.
        // docs를 읽어보니 계정을 인스트럭트의 마지막에서 닫는다고만 설명이 나와잇는데
        // 이러한 부분은 좀더 알아봐야 할 것 같습니다.
    )]
    pub user_todo : Box<Account<'info, TodoAccount>>,
}

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

pub fn bump(seeds:&[&[u8]], program_id : &Pubkey) -> u8{
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    // https://docs.rs/anchor-lang/0.13.2/anchor_lang/prelude/struct.Pubkey.html#method.find_program_address
    bump
}
