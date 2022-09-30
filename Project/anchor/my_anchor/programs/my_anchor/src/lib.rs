//anchor을 사용하는 이유는 다음과 같습니다.

// 1. 빠르게 작성 가능합니다.
// 2. 보안 검사를 자동으로 해줍니다.

// https://book.anchor-lang.com/introduction/what_is_anchor.html 

// anchor을 사용하기 위해 import해 옵니다.
use anchor_lang::prelude::*;

// 현재 이 프로그램의 id를 선업합니다.
// anchor build를 하게 되면 target폴더 내에 deploy가 나오게 된다.
// 그후 solana address -k <json파일 경로>를 입력해 준뒤에 해당 값을 여기에 주자.!!
declare_id!("B5vY2qT79BqKuSjPaT1p4uszdpTMcqGm9wabeKkpLjBp");

#[program] // anchor가 프로그램을 정의할떄 사용하는 특성입니다.
mod basic_1 {
    use super::*;
    // 이곳에 정의되는 함수들을 통해서 contract에 기록을 남길 수 있습니다.
    // -> business logic을 적는 곳이라고 말 할 수도 있습니다.

    // https://docs.rs/anchor-lang/latest/anchor_lang/context/struct.Context.html
    // 이곳을 참고하면 Context의 타입에 대해서 알 수 있습니다.
    // -> 쉽게 말하면 주소 정보를 담고 있는 타입입니다.
    pub fn initialize(ctx :Context<Initialize> , data : u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn update(ctx : Context<Update>, data : u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

// 들어오는 주소를 이곳에서 검증합니다.
#[derive(Accounts)] // 이런식의 구문을 쓰는 것은 간단합니다. MyAccount의 제네릭 타입을 지정해 주기 위해서 사용을 합니다.
pub struct Initialize<'info> {
    //'info같은 경우에는 AccountInfo를 가르키는 라이프사이클 변수 입니다.
    #[account(init, payer = user, space = 8 + 8)] 
    // 해당 구문 같은 경우에는 특정한 행동을 정의내리기 위해서 사용을 합니다.
    // 1. init -> 계정을 만듭니다.
    // 2. payer -> program에 계정을 만드는 user를 말합니다.
    // 3. space -> 계정의 공간으로 항상 8의 수치를 가지고 있고 여기에 데이터 사이즈를 더해줍니다.
    pub my_account : Account<'info, MyAccount>,
    // Account는 다음과 같이 두가지 제너릭 타입을 가지게 됩니다.
    // 1. 'info -> 일반적인 Account타입입니다.
    // 2. MyAccount -> 커스텀 타입으로 위에 있는 initialize함수 에서 my_accout를 호출할떄 나오는 데이터 입니다.
    #[account(mut)]
    // mut -> account의 데이터를 바꾸는 행위가 일어나기 떄문에 적어 줍니다.
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account : Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data : u64,
}