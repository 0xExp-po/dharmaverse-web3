use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NFTDATA {
    pub mint_address: Pubkey,
    pub updated_date: i64,
    pub reward: u64,
}

impl NFTDATA {
    pub const LEN: usize = 8 + 32 + 64 + 64 + 8;
}

#[account]
#[derive(Default)]
pub struct NFTDATAList {
    pub authority: Pubkey,
    pub nfts: Vec<NFTDATA>,
}

impl NFTDATAList {
    pub fn space(_num_items: usize) -> usize {
        8 + (NFTDATA::LEN * _num_items) // 8 bytes for authority + size of each NFTDATA entry * maximum number of NFTDATA entries
    }
}