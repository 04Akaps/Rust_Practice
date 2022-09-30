use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, approve, burn, close_account, freeze_account, initialize_account, initialize_mint,
    mint_to, revoke, set_authority, transfer, Approve, Burn, InitializeAccount, Mint, MintTo,
    SetAuthority, Token, TokenAccount, Transfer, *,
};

pub mod errors;

use errors::*;

declare_id!("Fi6agP72G3qHHidMNTsNpMrqv4DgqJGQr1iEDUrYuTiV");

#[program]
pub mod anchor_spl_token {
    use super::*;

    pub fn transfer(ctx: Context<TransferStruct>, amount: u64) -> Result<()> {
        let balance: u64 = ctx.accounts.sender_token.amount;
        msg!("starting tokens : {:?}", &balance);
        // 간단하게 sender의 associated account에 있는 token물량을 확인한다.

        require!(balance > amount, TodoError::NotEnoughBalanceFromSender);
        // 만약 가지고 있는 물량보다 적다면 error 발생

        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        // 이후 block으로 만들어둔 impl을 활용하여 해당 struct의 함수를 실행 시켜 준다.
        // cpi에 대해서는 좀더 공부가 필요하지만 일단 이런 느낌으로 쓴다는 것 정도로 넘어가였다.

        ctx.accounts.sender_token.reload()?;
        // 이후 해당 계정을 storage에서 reload해준다.

        msg!("remaining tokens  : {:?}", balance);

        Ok(())
    }

    pub fn mint_to(ctx: Context<MintStruct>, amount: u64) -> Result<()> {
        // let owner = &mut ctx.sender.authority.key();

        // 이곳에서 오직 owner만 실행할 수 있게 require문을 걸어야 한다.
        token::mint_to(ctx.accounts.mint_ctx(), amount)?;

        msg!("mint is success , {:?}", amount);

        Ok(())
    }

    pub fn burn(ctx: Context<BurnStruct>, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.burn_ctx(), amount)?;
        Ok(())
    }

    pub fn approve(ctx: Context<ApproveStruct>, amount: u64) -> Result<()> {
        token::approve(ctx.accounts.approve_ctx(), amount)?;
        Ok(())
    }

    pub fn set_authority(
        ctx: Context<AuthorityStruct>,
        authority_type: AuthorityType,
        new_authority: Pubkey,
    ) -> Result<()> {
        token::set_authority(
            ctx.accounts.set_authority_ctx(),
            authority_type.into(),
            Some(new_authority),
        );
        Ok(())
    }

    pub fn initialize_account(ctx: Context<InitializeStruct>) -> Result<()> {
        token::initialize_account(ctx.accounts.initialize_account_ctx());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintStruct<'info> {
    // 의문점은 mint함수에 대한 owner권한을 어떻게 설정하냐가 의문점
    pub sender: Signer<'info>,

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>, // mint를 받을 user의 주소로 인식

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferStruct<'info> {
    pub sender: Signer<'info>,
    // sender의 wallet
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // sender의 associated Address

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,
    // receiver의 associated account
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnStruct<'info> {
    // 의문점은 mint함수에 대한 owner권한을 어떻게 설정하냐가 의문점
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // mint를 받을 user의 주소로 인식

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ApproveStruct<'info> {
    // 의문점은 mint함수에 대한 owner권한을 어떻게 설정하냐가 의문점
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // mint를 받을 user의 주소로 인식

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitializeStruct<'info> {
    // 의문점은 mint함수에 대한 owner권한을 어떻게 설정하냐가 의문점
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // mint를 받을 user의 주소로 인식

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AuthorityStruct<'info> {
    // 의문점은 mint함수에 대한 owner권한을 어떻게 설정하냐가 의문점
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>, // mint를 받을 user의 주소로 인식

    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> MintStruct<'info> {
    fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.receiver_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}

impl<'info> TransferStruct<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.sender_token.to_account_info(),
                to: self.receiver_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}

impl<'info> BurnStruct<'info> {
    fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.mint.to_account_info(),
                from: self.sender_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}

impl<'info> ApproveStruct<'info> {
    fn approve_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Approve {
                to: self.receiver_token.to_account_info(),
                delegate: self.sender_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}

impl<'info> InitializeStruct<'info> {
    fn initialize_account_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitializeAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            InitializeAccount {
                account: self.sender.to_account_info(),
                mint: self.mint.to_account_info(),
                authority: self.sender.to_account_info(),
                rent: self.sender.to_account_info(),
                // 공식문세어는 rent라는 필드는 없는데... 일단 없다고 계속 에러가 뜨기 떄문에 대충 값을 집어 넣음
            },
        )
    }
}

impl<'info> AuthorityStruct<'info> {
    fn set_authority_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            SetAuthority {
                current_authority: self.sender.to_account_info(),
                account_or_mint: self.mint.to_account_info(),
            },
        )
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens,    // 새로운 토큰을 민팅할 권한
    FreezeAccount, // 계정을 잠글 권한
    AccountOwner,  // 주어진 토큰 주소의 오너
    CloseAccount,  // 토큰 계정을 잠글 권한
}

impl From<AuthorityType> for spl_token::instruction::AuthorityType {
    fn from(authority_ty: AuthorityType) -> spl_token::instruction::AuthorityType {
        match authority_ty {
            AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
            AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
            AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
        }
    }
}
