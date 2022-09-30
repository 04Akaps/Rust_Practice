use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("User cannot be created, missing data")]
    CannotCreateUser,

    #[msg("Video cannot be created, missing data")]
    CannotCreateVideo,

    #[msg("Cannot receiver more than 5 likes")]
    ReachedMaxLikes,

    #[msg("User has already liked the tweet")]
    UserLikedVideo,

    #[msg("Video with potentially bad content")]
    CannotComment,
}
