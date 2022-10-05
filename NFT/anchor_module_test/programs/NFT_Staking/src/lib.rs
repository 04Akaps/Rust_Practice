use anchor_lang::prelude::*;

declare_id!("BPrW9qJNafGsKTWraRecHapUXxQs8hbrR6VMDoLicfbL");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

        // let user = &mut ctx.accounts.user;

        my_account.account.user_address = *my_account.to_account_info().key;

        // 코드를 보면 다음과 같다.
        // 트랜잭션을 전송할떄 인자로 들어오는 my_account를 통해서 데이터가 저장이 되기 떄문에

        // signer는 고정이고 여기에 해당하는 데이터를 my_Account의 주소가 담고 있는 것과 같다.
        // 그러기 떄문에 해당 my_account값을 저장해 주고 후에 데이터를 불러올떄 해당 값을 통해서 불러와야 한다.

        // 생각해보니 굳이 다른 wallet의 address를 넣을 필요가 없다...
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 256)]
    // 기본적으로 8의 크기를 가져야 한다.
    // 그후 32를 더해준 이유는 payer를 user로 설정을 하였기 떄문에 해당 값을 추가한다. == PubKey
    // 마지막은 MyAccount의 데이터 크기
    // ** 중요한게 해당 space부분이 맞지 않으면 트랜잭션이 전송이 안된다... **
    pub my_account: Account<'info, MyAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
    pub account: AccountStorage,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AccountStorage {
    pub user_address: Pubkey,
}
