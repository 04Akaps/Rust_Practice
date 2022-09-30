use anchor_lang::prelude::*;

// 이번에는 두가지의 account Struct를 구성하여 활용할 예정

#[account]
pub struct UserProfile {
    pub authority : Pubkey,
    pub last_todo : u8,
    pub todo_count : u8
}

#[account]
pub struct TodoAccount {
    pub authority : Pubkey,
    pub index : u8,
    pub content : String,
    pub marked : bool
}