use anchor_lang::prelude::*;

// solana는 solidity와 다르게 동작하는 것이 특이합니다.
// Solidity같은 경우에는 하나의 Contract에 이제 모든 address, 데이터를 저장함으로써 동작을 하는데.
// Solana에게 Contract는 단순히 하나의 프로그램 로직에 불과합니다.
// 그러기 떄문에 해당 프로그래 == Contract를 실행하면 Contract와 연결된 하나의 새로운 Key를 만들고 그 Key와 실행자의 주소를 연결함으로써 동작합니다.

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_second {
    use super::*;

    pub fn start_stuff_off(ctx : Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        // 이곳에서는 base_account를 지정하는 이유는 간단하다.
        // 이전에는 my_account였지만 해당 행위는 단순히 Struct에 있는 변수값을 지정하는 것이기 떄문에 그냥 Struct에 있는 변수명을 입력해 주면 된다.
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link:String) -> Result<() {
        let base_account = &mut ctx.accounts.base_account;
        // basc
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link : gif_link.to_string(),
            user_address : *user.to_account_info().key,
        }

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {
    #[account(init, payer * user, space * 9000)]
    pub base_account : Account<'info, BaseAccount>,
    // 처음에는 해당 코드가 내가 한 행위를 서명하는 코드인줄 알았다. --> Signer떄문에..
    // 하지만 그것과는 별개로 해당 코드는 해당 프로그램 == SmartContract에 새로운 계정을 등록하는 코드로 인식을 하게 되었다.
    // 왜냐하면 만약 트랜잭션에 서명을 하는 코드라면 아래있는 AddGif에도 들어가야 하는 코드이기 떄문에
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account : Account<'info , BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link:String,
    pub user_Address : Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs : u64,
    pub gif_list : Vec<Itemstruct>,
}



