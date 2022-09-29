use anchor_lang::prelude::*;

// 전반적인 Error코드를 따로 두어서 활용할 예정

#[error_code]
pub enum TodoError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("Not Allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked
}