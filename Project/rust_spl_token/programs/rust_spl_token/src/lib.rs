use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};
// https://docs.rs/anchor-spl/latest/anchor_spl/token/index.html

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod rust_spl_token {
    use super::*;

    pub fn transfer(ctx: Context<TransferStruct>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.into(), amount);
        // transfer는 CpiContext를 인자로 받는다.
        // 이떄 into는 해당 값으로 변경해 주는 것으로 예를들면 이렇게 사용이 된다.
        // ley my_str  "hello";
        // let string3 : String = my_str.into();

        // CpiContext에 대한 내용은 너무 빈약하기 떄문에 후에 알아보도록..
        // https://docs.rs/anchor-spl/latest/anchor_spl/token/fn.transfer.html

        Ok(())
    }

    pub fn mint_to(ctx: Context<MintToStruct>, amount: u64) -> Result<()> {
        token::mint_to(ctx.accounts.into(), amount);

        Ok(())
    }

    pub fn burn(ctx: Context<BurnStruct>, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.into(), amount);

        Ok(())
    }

    pub fn set_authority(
        ctx: Context<SetAuthorityStruct>,
        authority_type: AuthorityType,
        new_authority: Pubkey,
    ) -> Result<()> {
        token::set_authority(
            ctx.accounts.into(),
            authority_type.into(),
            Some(new_authority),
        );

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens,    // 새로운 토큰을 민팅할 권한
    FreezeAccount, // 계정을 잠글 권한
    AccountOwner,  // 주어진 토큰 주소의 오너
    CloseAccount,  // 토큰 계정을 잠글 권한
}

#[derive(Accounts)]
pub struct TransferStruct<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub from: AccountInfo<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintToStruct<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnStruct<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetAuthorityStruct<'info> {
    #[account(signer)]
    pub current_authority: AccountInfo<'info>,

    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}

// **for create CpiContext **

// CpiContext를 만들고 해당 값을 함수를 실행할 떄 사용합니다.

impl<'a, 'b, 'c, 'info> From<&mut TransferStruct<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(
        accounts: &mut TransferStruct<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut MintToStruct<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut MintToStruct<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut BurnStruct<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Burn<'info>>
{
    fn from(accounts: &mut BurnStruct<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.mint.clone(),
            from: accounts.to.clone(), // to?
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut SetAuthorityStruct<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>
{
    fn from(
        accounts: &mut SetAuthorityStruct<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts.account_or_mint.clone(),
            current_authority: accounts.current_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
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
