use anchor_lang::prelude::*;

#[constant]
pub const NFT_SEED: &[u8] = b"nft";

#[constant]
pub const INTEREST_PER_NFT: u64 = 1000; // default limit interest per nft: 1000
