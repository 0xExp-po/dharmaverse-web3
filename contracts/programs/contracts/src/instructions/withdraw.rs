use crate::{constants::*, errors::*, state::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, transfer, Transfer},
};

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    require!(
        ctx.accounts.user_nft_account.amount == 1,
        Errors::InvalidOwner
    ); // verify nft owner

    let nfts = &mut ctx.accounts.nft_list.nfts;
    for item in nfts {
        if item.mint_address == ctx.accounts.nft.key() {
            let cur_timestamp = ctx.accounts.clock.unix_timestamp;
            let days = calculate_days_between_timestamps(item.updated_date, cur_timestamp);
            let amount = days * 5;

            require!(
                amount < item.reward,
                Errors::Invalidrewards
            );

            // send reward token to user
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(), 
                Transfer { 
                    from: ctx.accounts.contract_reward_account.to_account_info(), 
                    to: ctx.accounts.user_reward_account.to_account_info(), 
                    authority: ctx.accounts.user.to_account_info() 
                }), 
            amount)?;

            item.updated_date = cur_timestamp;
            let _ = item.reward.checked_sub(amount);
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [
            NFT_SEED
        ],
        bump,
    )]
    pub nft_list: Account<'info, NFTDATAList>,

    pub nft: Box<Account<'info, Mint>>,

    pub reward: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward,
        associated_token::authority = user,
    )]
    pub user_reward_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    contract_reward_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft,
        associated_token::authority = user,
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,

    /// Solana ecosystem accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
