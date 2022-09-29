use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod states;

use constant::*;
use error::*;
use states::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_anchor_fourth {
    use super::*;
}
