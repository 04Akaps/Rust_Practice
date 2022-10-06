use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use spl_token::instruction::AuthorityType;

declare_id!("5YJa8BpFJPQuAh29pGgo4NYfYRqQ66m5QHj2p4o5SF1r");

pub const GLOBAL_AUTHORITY_SEED: &str = "global-authority";

#[program]
pub mod nft_staking {
    use super::*;

    pub fn transfer(ctx : Context<TransferStruct>) -> Result<()> {

        // 단순히 transfer만 하면 된다...?
        let structV = ctx.accounts;
        token::transfer(
            structV.transfer_ctx(),
            1
        );

        Ok(())
    }

    // pub fn stake_nft(ctx: Context<StakeNFT>, global_bump: u8, staked_nft_bump: u8) -> Result<()> {
    //     //
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct TransferStruct<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + 8,
        // owner = system_program.key() // 굳이 적어주지 않아도 된다. == default값
    )]
    pub storage: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint : AccountInfo<'info>, // NFT에 대한 Token Address
    // 더미 입니다.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

// impl<'info> From<&mut StakeNFT<'info>> for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//     fn from(accounts: &mut StakeNFT<'info>) -> Self {
//         let cpi_accounts = SetAuthority {
//             account_or_mint: accounts.stake_nft.clone(),
//             current_authority: accounts.depositor.clone(),
//         };
//         let cpi_program = accounts.token_program.to_account_info();
//         CpiContext::new(cpi_program, cpi_accounts)
//     }
// }

impl<'info> TransferStruct<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.mint.to_account_info(),
                to: self.storage.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
}

// impl<'info> AuthorityStruct<'info> {
//     fn set_authority_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//         CpiContext::new(
//             self.token_program.to_account_info(),
//             SetAuthority {
//                 current_authority: self.sender.to_account_info(),
//                 account_or_mint: self.mint.to_account_info(),
//             },
//         )
//     }
// }

#[error_code]
pub enum ErrorCode {
    #[msg("not enough time to reclaim")]
    NotEnoughTime,
    #[msg("invalid metadata information")]
    NoMatchMetadata,
    #[msg("invalid token")]
    NoMatchSymbol,
}
