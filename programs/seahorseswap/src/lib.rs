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
        ProgramError::E007
    );

    require!(
        escrow.offering_token_account_pubkey == escrow_offering_token_account.key(),
        ProgramError::E008
    );

    require!(
        escrow.requesting_token_account_pubkey == original_requesting_token_account.key(),
        ProgramError::E009
    );

    require!(
        original_requesting_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E010
    );

    require!(
        original_offering_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E011
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
        ProgramError::E012
    );

    require!(
        escrow.requesting_token_account_pubkey == escrow_requesting_token_account.key(),
        ProgramError::E013
    );

    require!(
        escrow_requesting_token_account.owner == escrow.key(),
        ProgramError::E014
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
        ProgramError::E015
    );

    require!(
        escrow.requesting_token_account_pubkey == original_requesting_token_account.key(),
        ProgramError::E009
    );

    require!(
        escrow.offering_token_account_pubkey == original_offering_token_account.key(),
        ProgramError::E008
    );

    require!(
        original_requesting_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E010
    );

    require!(
        original_offering_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E011
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
        ProgramError::E016
    );

    require!(
        original_requesting_token_account.owner == final_offering_token_account.owner,
        ProgramError::E017
    );

    require!(
        final_offering_token_account.owner == escrow.requesting_wallet_pubkey,
        ProgramError::E018
    );

    require!(
        final_requesting_token_account.owner == escrow.offering_wallet_pubkey,
        ProgramError::E019
    );

    require!(
        escrow_offering_token_account.amount == (1 as u64),
        ProgramError::E020
    );

    require!(
        escrow_requesting_token_account.amount == (1 as u64),
        ProgramError::E021
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
    # [msg ("The offering signing pubkey does not match the authority found on the given offering token account")]
    E000,
    # [msg ("The requesting pubkey does not match the authority found on the given requesting token account")]
    E001,
    #[msg("The given offering token account supply must be equal to 1")]
    E002,
    #[msg("The given requesting token account supply must be equal to 1")]
    E003,
    # [msg ("The offering signer pubkey did not match the offering wallet pubkey found on the given escrow")]
    E004,
    # [msg ("The given escrow offering token account did not match the pubkey found on the given escrow")]
    E005,
    # [msg ("The authority found on the given escrow offering token account did not match the pubkey from the given escrow")]
    E006,
    #[msg(
        "The offering signer pubkey did not match the offering wallet pubkey on the given escrow"
    )]
    E007,
    # [msg ("The pubkey on the given escrow offering token account did not match the offering token account pubkey on the given escrow")]
    E008,
    # [msg ("The pubkey on the given escrow requesting token account did not match the requesting token account pubkey on the given escrow")]
    E009,
    # [msg ("The authority on the given original reqesting token account did not match the requesting wallet pubkey found on the given escrow")]
    E010,
    # [msg ("The authority on the given original offering token account did not match the offering wallet pubkey found on the given escrow")]
    E011,
    # [msg ("The requesting signer pubkey did not match the requesting wallet pubkey found on the given escrow")]
    E012,
    # [msg ("The given escrow requesting token account did not match the pubkey found on the given escrow")]
    E013,
    # [msg ("The authority found on the given escrow requesting token account did not match the pubkey from the given escrow")]
    E014,
    # [msg ("The requesting signer pubkey did not match the requesting wallet pubkey on the given escrow")]
    E015,
    # [msg ("The authority on the original offering token account did not match the authoity on the final requesting token account")]
    E016,
    # [msg ("The authority on the requesting token account did not match the authority on the final offering token account")]
    E017,
    # [msg ("The authority on the final offering token account did not match the requesting wallet pubkey found on the given escrow")]
    E018,
    # [msg ("The authority on the final requesting token account did not match the offering wallet pubkey from the given escrow")]
    E019,
    #[msg("The amount of tokens in the escrow offering token account should equal 1")]
    E020,
    #[msg("The amount of tokens in the escrow requesting token account should equal 1")]
    E021,
}
