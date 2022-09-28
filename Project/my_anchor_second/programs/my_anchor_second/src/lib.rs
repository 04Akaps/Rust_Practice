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
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link:String) -> Result<() {
        let base_account = &mut ctx.accounts.base_account;
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

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program : Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account : Account<'info , BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs : u64,
}

