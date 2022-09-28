use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_third {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


pub fn is_zero_account(account_info: &AccountInfo) -> bool{
    // 들어오는 주소값이 zero인지 체크하는 함수
    let account_data : &[u8] = &account_info.data.borrow();
    // AccountInfo의 data타입은 RefCel
    // 이중 borrow데이터가 의미하는 것은 wrap되어 있는 데이터를 가져오는 것을 의미한다.
    // -> https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html#method.borrow


    let len = account_data.len();


    for i in 0..len - 1{
        if account_data[i] != 0{
            return false;
        }
    };

    true
}