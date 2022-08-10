use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct Escrow {
    offering_wallet_pubkey: Pubkey,
    requesting_wallet_pubkey: Pubkey,
    offering_token_mint_pubkey: Pubkey,
    requesting_token_mint_pubkey: Pubkey,
    offering_token_account_pubkey: Pubkey,
    requesting_token_account_pubkey: Pubkey,
}

pub fn init_escrow_handler(
    mut ctx: Context<InitEscrow>,
    mut requesting_wallet_pubkey: Pubkey,
) -> Result<()> {
    let mut offering_signer = &mut ctx.accounts.offering_signer;
    let mut offering_token_mint = &mut ctx.accounts.offering_token_mint;
    let mut requesting_token_mint = &mut ctx.accounts.requesting_token_mint;
    let mut original_offering_token_account = &mut ctx.accounts.original_offering_token_account;
    let mut original_requesting_token_account = &mut ctx.accounts.original_requesting_token_account;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut escrow_offering_token_account = &mut ctx.accounts.escrow_offering_token_account;
    let mut escrow_requesting_token_account = &mut ctx.accounts.escrow_requesting_token_account;
    let mut escrow = escrow;

    escrow.offering_wallet_pubkey = offering_signer.key();

    escrow.requesting_wallet_pubkey = requesting_wallet_pubkey;

    let mut escrow_offering_token_account = escrow_offering_token_account;

    escrow.offering_token_mint_pubkey = offering_token_mint.key();

    escrow.requesting_token_mint_pubkey = requesting_token_mint.key();

    escrow.offering_token_account_pubkey = escrow_offering_token_account.key();

    escrow.requesting_token_account_pubkey = escrow_requesting_token_account.key();

    require!(
        offering_signer.key() == original_offering_token_account.owner,
        ProgramError::E000
    );

    require!(
        requesting_wallet_pubkey == original_requesting_token_account.owner,
        ProgramError::E001
    );

    require!(
        original_offering_token_account.amount == (1 as u64),
        ProgramError::E002
    );

    require!(
        original_requesting_token_account.amount == (1 as u64),
        ProgramError::E003
    );

    Ok(())
}

pub fn fund_escrow_offering_token_account_handler(
    mut ctx: Context<FundEscrowOfferingTokenAccount>,
) -> Result<()> {
    let mut offering_signer = &mut ctx.accounts.offering_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut original_offering_token_account = &mut ctx.accounts.original_offering_token_account;
    let mut escrow_offering_token_account = &mut ctx.accounts.escrow_offering_token_account;

    require!(
        escrow.offering_wallet_pubkey == offering_signer.key(),
        ProgramError::E004
    );

    require!(
        escrow.offering_token_account_pubkey == escrow_offering_token_account.key(),
        ProgramError::E005
    );

    require!(
        escrow_offering_token_account.owner == escrow.key(),
        ProgramError::E006
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: original_offering_token_account.to_account_info(),
                authority: offering_signer.to_account_info(),
                to: escrow_offering_token_account.to_account_info(),
            },
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn defund_escrow_offering_token_account_handler(
    mut ctx: Context<DefundEscrowOfferingTokenAccount>,
    mut escrow_bump: u8,
) -> Result<()> {
    let mut offering_signer = &mut ctx.accounts.offering_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut original_offering_token_account = &mut ctx.accounts.original_offering_token_account;
    let mut original_requesting_token_account = &mut ctx.accounts.original_requesting_token_account;
    let mut escrow_offering_token_account = &mut ctx.accounts.escrow_offering_token_account;

    require!(
        offering_signer.key() == escrow.offering_wallet_pubkey,
        ProgramError::E004
    );

    require!(
        escrow.offering_token_account_pubkey == escrow_offering_token_account.key(),
        ProgramError::E005
    );

    require!(
        escrow.requesting_token_account_pubkey == original_requesting_token_account.key(),
        ProgramError::E005
    );

    require!(
        original_requesting_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E007
    );

    require!(
        original_offering_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E008
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: escrow_offering_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: original_offering_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                original_offering_token_account.key().as_ref(),
                original_requesting_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn fund_escrow_requesting_token_account_handler(
    mut ctx: Context<FundEscrowRequestingTokenAccount>,
) -> Result<()> {
    let mut requesting_signer = &mut ctx.accounts.requesting_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut original_requesting_token_account = &mut ctx.accounts.original_requesting_token_account;
    let mut escrow_requesting_token_account = &mut ctx.accounts.escrow_requesting_token_account;

    require!(
        escrow.requesting_wallet_pubkey == requesting_signer.key(),
        ProgramError::E009
    );

    require!(
        escrow.requesting_token_account_pubkey == escrow_requesting_token_account.key(),
        ProgramError::E005
    );

    require!(
        escrow_requesting_token_account.owner == escrow.key(),
        ProgramError::E010
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: original_requesting_token_account.to_account_info(),
                authority: requesting_signer.to_account_info(),
                to: escrow_requesting_token_account.to_account_info(),
            },
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn defund_escrow_requesting_token_account_handler(
    mut ctx: Context<DefundEscrowRequestingTokenAccount>,
    mut escrow_bump: u8,
) -> Result<()> {
    let mut requesting_signer = &mut ctx.accounts.requesting_signer;
    let mut escrow = &mut ctx.accounts.escrow;
    let mut original_offering_token_account = &mut ctx.accounts.original_offering_token_account;
    let mut original_requesting_token_account = &mut ctx.accounts.original_requesting_token_account;
    let mut escrow_requesting_token_account = &mut ctx.accounts.escrow_requesting_token_account;

    require!(
        escrow.requesting_wallet_pubkey == requesting_signer.key(),
        ProgramError::E009
    );

    require!(
        escrow.requesting_token_account_pubkey == original_requesting_token_account.key(),
        ProgramError::E005
    );

    require!(
        escrow.offering_token_account_pubkey == original_offering_token_account.key(),
        ProgramError::E005
    );

    require!(
        original_requesting_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E007
    );

    require!(
        original_offering_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E008
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: escrow_requesting_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: original_requesting_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                original_offering_token_account.key().as_ref(),
                original_requesting_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

pub fn crank_swap_handler(mut ctx: Context<CrankSwap>, mut escrow_bump: u8) -> Result<()> {
    let mut escrow = &mut ctx.accounts.escrow;
    let mut original_offering_token_account = &mut ctx.accounts.original_offering_token_account;
    let mut original_requesting_token_account = &mut ctx.accounts.original_requesting_token_account;
    let mut escrow_offering_token_account = &mut ctx.accounts.escrow_offering_token_account;
    let mut escrow_requesting_token_account = &mut ctx.accounts.escrow_requesting_token_account;
    let mut final_offering_token_account = &mut ctx.accounts.final_offering_token_account;
    let mut final_requesting_token_account = &mut ctx.accounts.final_requesting_token_account;

    require!(
        original_offering_token_account.owner == final_requesting_token_account.owner,
        ProgramError::E011
    );

    require!(
        original_requesting_token_account.owner == final_offering_token_account.owner,
        ProgramError::E011
    );

    require!(
        final_offering_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E012
    );

    require!(
        final_requesting_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E013
    );

    require!(
        escrow_offering_token_account.amount == (1 as u64),
        ProgramError::E014
    );

    require!(
        escrow_requesting_token_account.amount == (1 as u64),
        ProgramError::E015
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: escrow_requesting_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: final_requesting_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                original_offering_token_account.key().as_ref(),
                original_requesting_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: escrow_offering_token_account.to_account_info(),
                authority: escrow.to_account_info(),
                to: final_offering_token_account.to_account_info(),
            },
            &[&[
                "escrow".as_bytes().as_ref(),
                original_offering_token_account.key().as_ref(),
                original_requesting_token_account.key().as_ref(),
                escrow_bump.to_le_bytes().as_ref(),
            ]],
        ),
        1 as u64,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub offering_signer: Signer<'info>,
    #[account(mut)]
    pub offering_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub requesting_token_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub original_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub original_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offering_signer,
        seeds = [
            "escrow".as_bytes().as_ref(),
            original_offering_token_account.key().as_ref(),
            original_requesting_token_account.key().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<Escrow>()
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init,
        payer = offering_signer,
        seeds = [
            "escrow-offered-token-account".as_bytes().as_ref(),
            original_offering_token_account.key().as_ref()
        ],
        bump,
        token::mint = offering_token_mint,
        token::authority = escrow
    )]
    pub escrow_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(
        init,
        payer = offering_signer,
        seeds = [
            "escrow-requested-token-account".as_bytes().as_ref(),
            original_requesting_token_account.key().as_ref()
        ],
        bump,
        token::mint = requesting_token_mint,
        token::authority = escrow
    )]
    pub escrow_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FundEscrowOfferingTokenAccount<'info> {
    #[account(mut)]
    pub offering_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub original_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct DefundEscrowOfferingTokenAccount<'info> {
    #[account(mut)]
    pub offering_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub original_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub original_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct FundEscrowRequestingTokenAccount<'info> {
    #[account(mut)]
    pub requesting_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub original_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct DefundEscrowRequestingTokenAccount<'info> {
    #[account(mut)]
    pub requesting_signer: Signer<'info>,
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub original_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub original_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct CrankSwap<'info> {
    #[account(mut)]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(mut)]
    pub original_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub original_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub final_offering_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub final_requesting_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod seahorseswap {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, requesting_wallet_pubkey: Pubkey) -> Result<()> {
        init_escrow_handler(ctx, requesting_wallet_pubkey)
    }

    pub fn fund_escrow_offering_token_account(
        ctx: Context<FundEscrowOfferingTokenAccount>,
    ) -> Result<()> {
        fund_escrow_offering_token_account_handler(ctx)
    }

    pub fn defund_escrow_offering_token_account(
        ctx: Context<DefundEscrowOfferingTokenAccount>,
        escrow_bump: u8,
    ) -> Result<()> {
        defund_escrow_offering_token_account_handler(ctx, escrow_bump)
    }

    pub fn fund_escrow_requesting_token_account(
        ctx: Context<FundEscrowRequestingTokenAccount>,
    ) -> Result<()> {
        fund_escrow_requesting_token_account_handler(ctx)
    }

    pub fn defund_escrow_requesting_token_account(
        ctx: Context<DefundEscrowRequestingTokenAccount>,
        escrow_bump: u8,
    ) -> Result<()> {
        defund_escrow_requesting_token_account_handler(ctx, escrow_bump)
    }

    pub fn crank_swap(ctx: Context<CrankSwap>, escrow_bump: u8) -> Result<()> {
        crank_swap_handler(ctx, escrow_bump)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("mismatch in token auth + signers")]
    E000,
    #[msg("mismatch in token auth + requested pubkey")]
    E001,
    #[msg("the supply must equal 1 for the offered token")]
    E002,
    #[msg("the supply must equal 1 for the requested token")]
    E003,
    #[msg("This swap escrow was not iniated by you.")]
    E004,
    #[msg("The escrow account does not match the given account.")]
    E005,
    #[msg("the given escrow_offering_token_account is now owned by the escrow")]
    E006,
    #[msg("The escrow requested pubkey does not match the authority for the given token account")]
    E007,
    #[msg("The escrow offered pubkey does not match the authority for the given token account")]
    E008,
    #[msg("This swap escrow was not requested to you.")]
    E009,
    #[msg("The given escrow_requesting_token_account is not owned by the escrow")]
    E010,
    #[msg("there is a mismatch in where the token should go")]
    E011,
    #[msg("the destination token account is now owned by the requested authority")]
    E012,
    #[msg("the destination token account is now owned by the offering authority")]
    E013,
    #[msg("the escrow account does not have the offered token")]
    E014,
    #[msg("the escrow account does not have the requested token")]
    E015,
}
