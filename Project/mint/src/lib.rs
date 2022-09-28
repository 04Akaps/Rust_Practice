use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        native_token::LAMPORTS_PER_SOL,
        program::invoke,
        pubkey::Pubkey,
        system_instruction,
    },
    spl_token::{
        instruction as token_instruction,
    },
    spl_associated_token_account::{
        instruction as token_account_instruction,
    }
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    println!("{:?}", accounts_iter);

    let minted_wallet = next_account_info(accounts_iter)?;
 

    msg!("Mint : {}", minted_wallet.key);

    invoke(&system_instruction::create_account(&minted_wallet.key, &minted_wallet.key, LAMPORTS_PER_SOL, 82, &minted_wallet.key), &[minted_wallet.clone(), minted_wallet.clone(), minted_wallet.clone()])?;

    // 1번쨰는 signer, LAMPORTS_PER_SOL 모름 , 82 : 스탠다드 민트 사이즈, 마지막 : token Owner


    msg!("Creating mint account...");
    msg!("Initializing mint account...");
    msg!("Creating token account...");
    msg!("Minting token to token account...");
    msg!("Token mint process completed...");

    Ok(())
}