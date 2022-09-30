use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;

pub mod constant;
pub mod error;
pub mod states;
pub mod util;

use constant::*;
use error::*;
use states::{StateAccount, UserAccount, VideoAccount, CommentAccount};
use util::bump;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_fourth {
    use super::*;

    pub fn init_state_account(ctx: Context<InitStateAccount>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.authority = ctx.accounts.authority.key();
        state.video_count = 0;

        Ok(())
    }

    pub fn create_user(
        ctx: Context<CreateUser>,
        _username: String,
        _user_image_url: String,
    ) -> Result<()> {
        if _username.trim().is_empty() || _user_image_url.trim().is_empty() {
            // 인자값이 빈값이면 에러를 리턴
            return Err(Errors::CannotCreateUser.into());
            // into가 들어간 이유는 들어오는 타입에 맞는 타입으로 변환해서 에러를 보내줄떄 사용 한다.
            //https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
        }

        let user = &mut ctx.accounts.user;

        user.user_wallet_address = ctx.accounts.authority.key();

        user.user_name = _username;
        user.user_profile_image_url = _user_image_url;

        msg!("User Added"); //로그를 남기기
        sol_log_compute_units(); // 얼마나 계산된 값이 남아 잇는지 로그를 남기는 또다른 방법

        Ok(())
    }

    pub fn create_video(
        ctx: Context<CreateVideo>,
        _description: String,
        _video_url: String,
        _creator_name: String,
        _creator_url: String,
    ) -> Result<()> {
        msg!(&_description);

        if _description.trim().is_empty() || _video_url.trim().is_empty() {
            return Err(Errors::CannotCreateVideo.into());
        }

        let state = &mut ctx.accounts.state;
        let video = &mut ctx.accounts.video;

        video.authority = ctx.accounts.authority.key();

        video.description = _description;
        video.video_url = _video_url;
        video.creator_name = _creator_name;
        video.creator_url = _creator_url;

        video.comment_count = 0;
        video.index = state.video_count;
        video.creator_time = ctx.accounts.clock.unix_timestamp;

        video.likes = 0;
        video.remove = 0;

        state.video_count = state.video_count + 1;
        // state.video_count += 1;
        // 이렇게도 작성 가능

        msg!("Video Created!!");
        sol_log_compute_units();
        Ok(())
    }

    pub fn create_comment(
        ctx : Context<CreateComment>,
        _text : String,
        _commenter_name : String,
        _commenter_url : String,
    ) -> Result<()> {

        let video = &mut ctx.accounts.video;

        if _text.trim().is_empty() || _commenter_name.trim().is_empty()|| video.remove <= -500 {
            return Err(Errors::CannotComment.into());
        }

        let comment = &mut ctx.accounts.comment;

        comment.authority = ctx.accounts.authority.key();

        comment.text = _text;
        comment.commenter_name = _commenter_name;
        comment.commenter_url = _commenter_url;

        comment.index = video.comment_count;
        comment.video_time = ctx.accounts.clock.unix_timestamp;

        video.comment_count +=1;

        
        Ok(())
    }

    pub fn like_video(ctx : Context<LikeVideo> )-> Result<()> {
        let video = &mut ctx.accounts.video;

        if video.likes == NUMBER_OF_ALLOWED_LIKES {
            return Err(Errors::ReachedMaxLikes.into());
        }

        if video.remove == -500{
            return Err(Errors::CannotComment.into());
        }

        let mut iter = video.people_who_liked.iter();
        // iter는 반복자를 만들어 주는 메서드 입니다.
        // https://rinthel.github.io/rust-lang-book-ko/ch13-02-iterators.html

        let user_liking_video = ctx.accounts.authority.key();

        if iter.any(|&v| v == user_liking_video){
            // 참조자를 통해서 해당 v값에 publicKet가 있는지를 확인하는 역할입니다.
            // https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html
            return Err(Errors::UserLikedVideo.into());
        }

        video.likes += 1;
        video.people_who_liked.push(user_liking_video);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStateAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [b"state".as_ref(), authority.key().as_ref()],
        // as_ref와 borrow는 유사하게 동작하지만 살짝 다릅니다.
        // as_ref는 변환의 목적, borrow는 빌림의 목적에 의미를 둡니다.
        // 예를들면 String은 borrow를 통해서 리터럴 타입 &str을 얻을 수 있게 됩니다.
        // 이에 반해 as_ref는 어떤 타입을 다른 타입으로 변환하여 쓰기 위한 곳에 사용이 됩니다.
        // https://doc.rust-lang.org/std/convert/trait.AsRef.html
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<StateAccount>()
    )]
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"state".as_ref(), authority.key().as_ref()],
        // b''은 바이트 리터럴을 의미합니다.
        bump,
        has_one = authority,
    )]
    pub state: Account<'info, StateAccount>,

    #[account (
        init,
        seeds = [b"user".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserAccount>() + USER_NAME_LENGTH 
    )]
    pub user: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct CreateVideo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"state".as_ref(), authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub state: Account<'info, StateAccount>,

    #[account (
        init,
        seeds = [b"video".as_ref(), authority.key().as_ref()],
        space = 8 + std::mem::size_of::<VideoAccount>() + VIDE_URL_LENGTH + TEXT_LENGTH,
        bump,
        payer = authority
    )]
    pub video: Account<'info, VideoAccount>,

    pub clock : Sysvar<'info, Clock>
}

#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account (
        init,
        seeds = [b"comment".as_ref(), authority.key().as_ref()],
        space = 8 + std::mem::size_of::<CommentAccount>(),
        bump,
        payer = authority
    )]
    pub comment: Account<'info, CommentAccount>,

    #[account (
        mut,
        seeds = [b"video".as_ref(), authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub video: Account<'info, VideoAccount>,

    pub clock : Sysvar<'info, Clock>
}

#[derive(Accounts)]
pub struct LikeVideo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,


    #[account (
        mut,
        seeds = [b"video".as_ref(), authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub video: Account<'info, VideoAccount>,
}