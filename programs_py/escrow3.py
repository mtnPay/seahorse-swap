# escrow3
# Built with Seahorse v0.1.2

from re import A
import string
from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Escrow(Account):
    offered_pubkey: Pubkey
    requested_pubkey: Pubkey
    offered_token_mint_pubkey: Pubkey
    requested_token_mint_pubkey: Pubkey
    offered_token_account_pubkey: Pubkey
    requested_token_account_pubkey: Pubkey


@instruction
def init_escrow(
    # the initiater of the swap
    offerer_signer: Signer,
    # the pubkey of the wallet that the iniater wants to swap with
    requested_pubkey: Pubkey,
    # the token that the initiater wants to give
    offered_token_mint: TokenMint,
    # the token that the initiater wants to receive
    requested_token_mint: TokenMint,
    # the token account that currently holds the token the initiater wants to give
    offered_holder_token_account: TokenAccount,
    # the token account that currently holds the token the initiater wants to receive
    requested_holder_token_account: TokenAccount,
    # the account that the escrow will be initiated to
    escrow: Empty[Escrow],
    # the new account that the offered token will be escrowed into
    new_offered_token_account: Empty[TokenAccount],
    # # the new account that the requested token will be escrowed into
    new_requested_token_account: Empty[TokenAccount],
):

    escrow = escrow.init(
        payer=offerer_signer,
        seeds=[
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account
        ]
    )
    escrow.offered_pubkey = offerer_signer.key()
    escrow.requested_pubkey = requested_pubkey

    new_offered_token_account = new_offered_token_account.init(
        payer=offerer_signer,
        seeds=['escrow-offered-token-account', offered_holder_token_account],
        mint=offered_token_mint,
        authority=escrow
    )

    new_requested_token_account.init(
        payer=offerer_signer,
        seeds=['escrow-requested-token-account',
               requested_holder_token_account],
        mint=requested_token_mint,
        authority=escrow
    )

    escrow.offered_token_mint_pubkey = offered_token_mint.key()
    escrow.requested_token_mint_pubkey = requested_token_mint.key()

    escrow.offered_token_account_pubkey = new_offered_token_account.key()
    escrow.requested_token_account_pubkey = new_requested_token_account.key()


@instruction
def fund_offered_escrow(
    offerer_signer: Signer,
    escrow: Escrow,
    offered_holder_token_account: TokenAccount,
    new_offered_token_account: TokenAccount
):

    assert escrow.offered_pubkey == offerer_signer.key(
    ), 'This swap escrow was not iniated by you.'

    assert escrow.offered_token_account_pubkey == new_offered_token_account.key(
    ), 'The escrow account does not match the given account.'

    offered_holder_token_account.transfer(
        offerer_signer,
        new_offered_token_account,
        u64(1)
    )


@instruction
def defund_offered_escrow(
    offered_signer: Signer,
    escrow_bump: u8,
    escrow: Escrow,
    offered_holder_token_account: TokenAccount,
    requested_holder_token_account: TokenAccount,
    new_offered_token_account: TokenAccount
):

    assert offered_signer.key(
    ) == escrow.offered_pubkey, 'This swap escrow was not iniated by you.'

    assert escrow.offered_token_account_pubkey == new_offered_token_account.key(
    ), 'The escrow account does not match the given account.'

    new_offered_token_account.transfer(
        escrow,
        offered_holder_token_account,
        u64(1),
        [
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account,
            escrow_bump
        ]
    )


@instruction
def fund_requested_escrow(
    requested_signer: Signer,
    escrow: Escrow,
    requested_holder_token_account: TokenAccount,
    new_requested_token_account: TokenAccount
):

    assert escrow.requested_pubkey == requested_signer.key(
    ), 'This swap escrow was not requested to you.'

    # assert escrow.requested_token_account_pubkey == new_requested_token_account.key(
    # ), 'The escrow account does not match the given account.'

    requested_holder_token_account.transfer(
        requested_signer,
        new_requested_token_account,
        u64(1)
    )


@instruction
def defund_requested_escrow(
    requested_signer: Signer,
    escrow_bump: u8,
    escrow: Escrow,
    offered_holder_token_account: TokenAccount,
    requested_holder_token_account: TokenAccount,
    new_requested_token_account: TokenAccount
):

    assert escrow.requested_pubkey == requested_signer.key(
    ), 'This swap escrow was not requested to you.'

    # assert escrow.requested_token_account_pubkey == new_requested_token_account.key(
    # ), 'The escrow account does not match the given account.'

    new_requested_token_account.transfer(
        escrow,
        requested_holder_token_account,
        u64(1),
        [
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account,
            escrow_bump
        ]
    )


@instruction
def crank_swap(
    payer: Signer,
    escrow_bump: u8,
    escrow: Escrow,
    offered_holder_token_account: TokenAccount,
    requested_holder_token_account: TokenAccount,
    new_offered_token_account: TokenAccount,
    new_requested_token_account: TokenAccount,
    final_offered_token_account: TokenAccount,
    final_requested_token_account: TokenAccount
):

    assert escrow.requested_token_account_pubkey == new_requested_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert escrow.offered_token_account_pubkey == new_offered_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert final_offered_token_account.authority(
    ) == escrow.requested_pubkey, 'COME UP WITH GOOD ERROR MESSAGE'

    assert final_requested_token_account.authority(
    ) == escrow.offered_pubkey, 'COME UP WITH GOOD ERROR MESSAGE'

    new_requested_token_account.transfer(
        escrow,
        final_requested_token_account,  # need to make an account for the offerer
        u64(1),
        [
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account,
            escrow_bump
        ]
    )

    new_offered_token_account.transfer(
        escrow,
        final_offered_token_account,  # need to make an account for the offerer
        u64(1),
        [
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account,
            escrow_bump
        ]
    )
