use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("InvalidOwner")]
    InvalidOwner,
    #[msg("Exceed limit rewards")]
    Invalidrewards,
}
