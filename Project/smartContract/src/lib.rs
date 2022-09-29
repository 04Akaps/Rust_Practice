use borsh::{BorshDeserialize, BorshSerialize};
// sequelize도구 - 직렬화 및 역 직렬화

use solana_program::{
    account_info::{next_account_info, AccountInfo}, // 계정 정보에 관한 모듈
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,      // smartContact의 id로 deploy가 되면 생성이 된다.
    accounts: &[AccountInfo], //   user의 address
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("hellow world rust program entrypoint");

    let accounts_iter = &mut accounts.iter();

    msg!("accounts_iter : {:?}", accounts_iter);

    let account = next_account_info(accounts_iter)?;
    // AccountInfo 타입을 return 한다.
    msg!("account : {:?}", account);

    if account.owner != program_id {
        msg!("account does not have thr correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("account data : {:?}", account.data);

    msg!("account data borrow : {:?}", account.data.borrow());
    msg!("account data borrow & : {:?}", &account.data.borrow());

    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    msg!("greeting_account : {:?}", greeting_account);

    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
