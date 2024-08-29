use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

pub fn initialize(ctx: Context<Initialize>, nfts: Vec<Pubkey>) -> Result<()> {
    let nft_list = &mut ctx.accounts.nft_list;

    nft_list.authority = ctx.accounts.payer.key();
    nft_list.nfts = nfts.iter().map(|nft| NFTDATA {
        mint_address: *nft,
        updated_date: ctx.accounts.clock.unix_timestamp,
        reward: INTEREST_PER_NFT,
    }).collect();

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed, 
        seeds = [
            NFT_SEED
        ],
        bump,
        payer = payer, 
        space = NFTDATAList::space(5))]
    pub nft_list: Account<'info, NFTDATAList>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    }