use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct Escrow {
    offered_pubkey: Pubkey,
    requested_pubkey: Pubkey,
    offered_token_mint_pubkey: Pubkey,
    requested_token_mint_pubkey: Pubkey,
    offered_token_account_pubkey: Pubkey,
    requested_token_account_pubkey: Pubkey,
}

pub fn init_escrow_handler(
    mut ctx: Context<InitEscrow>,
    mut requested_pubkey: Pubkey,
) -> Result<()> {
    let mut offerer_signer = &mut ctx.accounts.offerer_signer;
    let mut offered_token_mint = &mut ctx.accounts.offered_token_mint;
    let mut requested_token_mint = &mut ctx.accounts.requested_token_mint;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut requested_holder_token_account = &mut ctx.accounts.requested_holder_token_account;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;
    let mut new_requested_token_account = &mut ctx.accounts.new_requested_token_account;
    let mut escrow = escrow;

    escrow.offered_pubkey = offerer_signer.key();

    escrow.requested_pubkey = requested_pubkey;

    let mut new_offered_token_account = new_offered_token_account;

    escrow.offered_token_mint_pubkey = offered_token_mint.key();

    escrow.requested_token_mint_pubkey = requested_token_mint.key();

    escrow.offered_token_account_pubkey = new_offered_token_account.key();

    escrow.requested_token_account_pubkey = new_requested_token_account.key();

    Ok(())
}

pub fn fund_escrow_handler(mut ctx: Context<FundEscrow>) -> Result<()> {
    let mut offerer_signer = &mut ctx.accounts.offerer_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut offered_holder_token_account = &mut ctx.accounts.offered_holder_token_account;
    let mut new_offered_token_account = &mut ctx.accounts.new_offered_token_account;

    require!(
        escrow.offered_pubkey == offerer_signer.key(),
        ProgramError::E000
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: offered_holder_token_account.to_account_info(),
                authority: offerer_signer.to_account_info(),
                to: new_offered_token_account.to_account_info(),
            },
        ),
        1 as u64,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub offerer_signer: Signer<'info>,
    #[account(mut)]
    pub offered_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub requested_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub requested_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow".as_bytes().as_ref(),
            offered_holder_token_account.key().as_ref(),
            requested_holder_token_account.key().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<Escrow>()
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow-offered-token-account".as_bytes().as_ref(),
            offered_holder_token_account.key().as_ref()
        ],
        bump,
        token::mint = offered_token_mint,
        token::authority = escrow
    )]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offerer_signer,
        seeds = [
            "escrow-requested-token-account".as_bytes().as_ref(),
            requested_holder_token_account.key().as_ref()
        ],
        bump,
        token::mint = requested_token_mint,
        token::authority = escrow
    )]
    pub new_requested_token_account: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FundEscrow<'info> {
    #[account(mut)]
    pub offerer_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub offered_holder_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub new_offered_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod escrow3 {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, requested_pubkey: Pubkey) -> Result<()> {
        init_escrow_handler(ctx, requested_pubkey)
    }

    pub fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()> {
        fund_escrow_handler(ctx)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("This swap escrow was not iniated by you.")]
    E000,
}
