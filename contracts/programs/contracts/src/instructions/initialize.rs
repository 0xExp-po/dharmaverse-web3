use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
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

    pub reward: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = reward,
        associated_token::authority = payer,
    )]
    pub contract_reward_account: Box<Account<'info, TokenAccount>>,

    /// Solana ecosystem accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    }