use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::*;

// https://docs.rs/anchor-spl/latest/anchor_spl/token/index.html

// splToken타입에는 여러개가 있기 떄문에 사용할떄마다 하나씩 뜯어보겠습니다.

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_token_first {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>, value: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        // https://docs.rs/anchor-spl/latest/anchor_spl/token/struct.MintTo.html

        // accountInfo는 AccountInfo struct로 타입을 바꾸는 것을 의미합니다.
        // MintTo 함수는 3가지 인자의 타입으로 AccountInfo타입을 받고 있기 떄문에 바꿔 주어야 합니다.

        // 한가지 궁금한 점은 애초에 authority는 AcountInfo타입인데 왜 전환을 시키는지 이해가 안갑니다.

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // https://docs.rs/anchor-lang/0.5.0/anchor_lang/prelude/struct.CpiContext.html
        // 이 부분은 따로 설명이 나와있지 않아.. 모르겠습니다.

        token::mint_to(cpi_ctx, value);
        // https://docs.rs/anchor-spl/0.5.0/anchor_spl/token/fn.mint_to.html
        // 동일하게 따로 설명이 없습니다..

        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, value: u64) -> Result<()> {
        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, value);

        Ok(())
    }
}

// 일단 이전에 작성하던 방식과 다른 타입들이 몇가지 등장을 하였기 떄문에 조금 맛보고 넘어가겠습니다.

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: AccountInfo<'info>,
    // 참고한 docs링크는 이와 같습니다.

    // 1. https://docs.rs/anchor-lang/latest/anchor_lang/prelude/struct.AccountInfo.html

    // 2. https://docs.rs/anchor-lang/0.18.0/anchor_lang/struct.Signer.html

    // 3. https://docs.rs/anchor-lang/0.16.0/anchor_lang/struct.UncheckedAccount.html

    /*

    -- Signer vs AccountInfo --

    일단 기본적으로 authority는 이떄까지 Signer<'info>로 사용을 해왔는데 이번에는 AccountInfo를 사용을 하였습니다.

    사실 두 타입에는 그렇게 큰 차이는 없다고 생각을 합니다.

    docs롤 보면 알 수 있듯이 AccountInfo는 Signer에 비해서 좀더 많은 메서드를 담고 있고 Signer는 이에 반해 빈약합니다.

    하지만 Signer에서도 AccountInfo 메서드가 활용이 가능하다는 것을 docs를 통해서 알 수 잇고 실제로도 사용을 해봤기 떄문에 크게 신경쓰지 않고 넘어갔습니다.
    - 대표적으로 Key()가 있습니다.


    -- UncheckedAccount --

    이떄까지는 Account타입으로 사용을 한것을 이번에는 UncheckedAccount타입으로 적었습니다.

    Account는 따른 sturct타입을 집어 넣을떄 사용을 합니다.
    -> https://docs.rs/anchor-lang/0.11.0/anchor_lang/derive.Accounts.html

    반드시 특성을 위에 적어주어야 합니다.

    반대로 UncheckedAccount는 아무런 struct를 활용하지 않을떄에 사용을 합니다.
    그러기 떄문에 wrap타입이라고도 하며 해당 값에 뭐든지 저장 할 수 있는 타입이라고 할 수 ㅅ있습니다.
    - 데이터를 가져올떄는 borrow를 사용합니다.
    -> https://docs.rs/anchor-lang/0.16.0/anchor_lang/struct.UncheckedAccount.html

    **공부하는 과정이기 떄문에 틀린점이 많을 수 있습니다..!**
    */
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub from: UncheckedAccount<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub from_authority: Signer<'info>,
}
