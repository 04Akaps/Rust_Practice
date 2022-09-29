use anchor_lang::prelude::*;

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    // https://docs.rs/anchor-lang/0.13.2/anchor_lang/prelude/struct.Pubkey.html#method.find_program_address
    bump
}
