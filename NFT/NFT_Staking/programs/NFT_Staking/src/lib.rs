use anchor_lang::prelude::*;
use anchor_spl::token::{self, SetAuthority, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

// https://github.com/Brent-Jeremy/solana-nft-staking

declare_id!("5YJa8BpFJPQuAh29pGgo4NYfYRqQ66m5QHj2p4o5SF1r");

const TRANSIENT_NFT_STAKE_SEED_PREFIX: &[u8] = b"transient";
pub const METAPLEX_PROGRAM_ID: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
// metaPlex를 통해서 mint하면 해당 값에서 mint권한이 있는 계정을 만들어 주고, 이후 그 계정에서 Mint를 진행하는 방식
//  https://solscan.io/token/DtoUoANCQ4vws4kCXWy4qiXmWzhDVXXPX6f7bvmBWNiM?cluster=devnet

pub const SYMBOL: &[u8] = b"HVORIGINS";

#[program]
pub mod nft_staking {
    use super::*;

    pub fn stake_nft(ctx: Context<StakeNFT>, symbol: String) -> Result<()> {
        let metaplex_pubkey = METAPLEX_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("Failed to parse Metaplex Program Id");

        // metaplex의 주소를 가져온다

        let mint = *ctx.accounts.mint.key;

        let seeds = &[
            "metadata".as_bytes(), // 특정 타입을 제네릭 타입으로 변경
            metaplex_pubkey.as_ref(),
            mint.as_ref(),
        ];

        let (metadata_pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);
        // pubKey와 u8이 resturn 된다.
        // https://stackoverflow.com/questions/68878330/what-is-the-seeds-in-creating-account-or-finding-the-account-in-solana-and-coul

        if metadata_pda != *ctx.accounts.metadata.key {
            return Err(ErrorCode::NoMatchMetadata.into());
        }

        if symbol.as_bytes() != SYMBOL {
            return Err(ErrorCode::NoMatchSymbol.into());
        }

        let (pda, _bump_seed) = Pubkey::find_program_address(
            &[
                &TRANSIENT_NFT_STAKE_SEED_PREFIX[..],
                ctx.accounts.depositor.key.as_ref(),
                ctx.accounts.mint.key.as_ref(),
            ],
            ctx.program_id,
        );

        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;

        {
            let store = &mut ctx.accounts.store;
            let list = &mut ctx.accounts.list;
            list.items.resize(
                store.staked_count as usize + 1,
                StakeItem {
                    owner: *ctx.accounts.depositor.key,
                    token_mint: *ctx.accounts.mint.key,
                    holder: *ctx.accounts.stake_nft.key,
                    stake_time: ctx.accounts.clock.unix_timestamp,
                },
            );
        }

        {
            let store = &mut ctx.accounts.store;
            store.staked_count += 1;
        }
        Ok(())
    }

    pub fn check_stake_nft(ctx: Context<CheckNFT>) -> Result<()> {
        let metaplex_pubkey = METAPLEX_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("Failed to parse Metaplex Program Id");

        msg!("{:?}", metaplex_pubkey);

        let mint = *ctx.accounts.mint.key;

        let seeds = &[
            "metadata".as_bytes(), // 특정 타입을 제네릭 타입으로 변경
            metaplex_pubkey.as_ref(),
            mint.as_ref(),
        ];

        let (metadata_pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);
        // test를 진행해 보았지만 metadata_pda는 아무런 상관이 없는 주소가 나오게 된다.

        let store = &mut ctx.accounts.store;

        store.address = metadata_pda;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckNFT<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 32)]
    pub store: Account<'info, CheckStore>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct CheckStore {
    pub address: Pubkey,
}

#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub store: Account<'info, StakeStore>,
    #[account(mut)]
    pub list: Account<'info, StakeList>,
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    #[account(mut)]
    pub stake_nft: AccountInfo<'info>,

    pub metadata: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakeStore {
    pub is_initialized: bool,
    pub payer: Pubkey,
    pub staked_count: u64,
    pub stake_list: Pubkey,
    pub max_items: u64,
}

impl StakeStore {
    pub const LEN: usize = 1 + 32 + 8 + 32 + 8;
}

#[account]
pub struct StakeList {
    pub items: Vec<StakeItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct StakeItem {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub holder: Pubkey,
    pub stake_time: i64,
}

impl<'info> From<&mut StakeNFT<'info>> for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
    fn from(accounts: &mut StakeNFT<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts.stake_nft.clone(),
            current_authority: accounts.depositor.clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("not enough time to reclaim")]
    NotEnoughTime,
    #[msg("invalid metadata information")]
    NoMatchMetadata,
    #[msg("invalid token")]
    NoMatchSymbol,
}
