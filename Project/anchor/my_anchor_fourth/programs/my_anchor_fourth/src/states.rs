use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub user_wallet_address: Pubkey,
    pub user_name: String,
    pub user_profile_image_url: String,
}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub video_count: u64,
}

#[account]
pub struct CommentAccount {
    pub authority: Pubkey,
    pub text: String,
    pub commenter_name: String,
    pub commenter_url: String,
    pub index: u64,
    pub video_time: i64,
}

#[account]
pub struct VideoAccount {
    pub authority: Pubkey,
    pub description: String,
    pub video_url: String,
    pub creator_name: String,
    pub creator_url: String,
    pub comment_count: u64,
    pub index: u64,
    pub creator_time: i64,
    pub people_who_liked: Vec<Pubkey>,
    pub likes: u8,
    pub remove: i64,
}
