

use solana_program::{
    account_info::{next_account_info, AccountInfo}, // 계정 정보에 관한 모듈
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id : &Pubkey, // smartContact의 id로 deploy가 되면 생성이 된다.
    accounts : &[AccountInfo], //   user의 address
    _instruction_data : &[u8], 
) -> ProgramResult {
    msg!("hello, world");

    Ok(())
}