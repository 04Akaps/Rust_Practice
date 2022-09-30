use anchor_lang::prelude::*;

// 전반적인 Error코드를 따로 두어서 활용할 예정

#[error_code]
pub enum TodoError {
    #[msg("sender's balance Error")]
    NotEnoughBalanceFromSender,
}
