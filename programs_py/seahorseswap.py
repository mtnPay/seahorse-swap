# seahorseswap
# Built with Seahorse v0.1.2

from re import A
from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Escrow(Account):
    # pubkey of the wallet offering the swap
    offered_pubkey: Pubkey
    # pubkey of the wallet who is being requested to swap
    requested_pubkey: Pubkey
    # mint of the token being offered
    offered_token_mint_pubkey: Pubkey
    # mint of the token being requested
    requested_token_mint_pubkey: Pubkey
    # token account of the token being offered
    offered_token_account_pubkey: Pubkey
    # token account of the token being requested
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

    assert offerer_signer.key() == offered_holder_token_account.authority(
    ), 'mismatch in token auth + signers'

    assert requested_pubkey == requested_holder_token_account.authority(
    ), 'mismatch in token auth + requested pubkey'

    assert offered_holder_token_account.amount == 1, 'the supply must equal 1 for the offered token'

    assert requested_holder_token_account.amount == 1, 'the supply must equal 1 for the requested token'


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

    assert new_offered_token_account.authority() == escrow.key(
    ), 'the given new_offered_token_account is now owned by the escrow'

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

    assert escrow.requested_token_account_pubkey == new_requested_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert new_requested_token_account.authority() == escrow.key(
    ), 'The given new_requested_token_account is not owned by the escrow'

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
    '''
    - Tests to write
    1. The requsted signer is the same as the requested pubkey on the escrow contract
    2. The given requested token account pubkey matched the requested token escrow account pubkey 
    3. The given offered_holder_token_account authority matches the escrow's offered pubkey
    4. The given requested_holder_token_account authiority matches the escrow's requested pubkey
    '''

    assert escrow.requested_pubkey == requested_signer.key(
    ), 'This swap escrow was not requested to you.'

    assert escrow.requested_token_account_pubkey == requested_holder_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert escrow.offered_token_account_pubkey == offered_holder_token_account.key(
    ), 'The escrow account does not match the given account.'

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
    escrow_bump: u8,
    escrow: Escrow,
    offered_holder_token_account: TokenAccount,
    requested_holder_token_account: TokenAccount,
    new_offered_token_account: TokenAccount,
    new_requested_token_account: TokenAccount,
    final_offered_token_account: TokenAccount,
    final_requested_token_account: TokenAccount
):

    assert offered_holder_token_account.authority() == final_requested_token_account.authority(
    ), 'there is a mismatch in where the token should go'

    assert requested_holder_token_account.authority() == final_offered_token_account.authority(
    ), 'there is a mismatch in where the token should go'

    assert final_offered_token_account.authority(
    ) == escrow.requested_pubkey, 'the destination token account is now owned by the requested authority'

    assert final_requested_token_account.authority(
    ) == escrow.offered_pubkey, 'the destination token account is now owned by the offering authority'

    assert new_offered_token_account.amount == 1, 'the escrow account does not have the offered token'

    assert new_requested_token_account.amount == 1, 'the escrow account does not have the requested token'

    new_requested_token_account.transfer(
        escrow,
        final_requested_token_account,
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
        final_offered_token_account,
        u64(1),
        [
            'escrow',
            offered_holder_token_account,
            requested_holder_token_account,
            escrow_bump
        ]
    )
