# seahorseswap
# Built with Seahorse v0.1.2

from re import A
from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Escrow(Account):
    offering_wallet_pubkey: Pubkey
    requesting_wallet_pubkey: Pubkey
    offering_token_mint_pubkey: Pubkey
    requesting_token_mint_pubkey: Pubkey
    offering_token_account_pubkey: Pubkey
    requesting_token_account_pubkey: Pubkey


@instruction
def init_escrow(
    offering_signer: Signer,
    requesting_wallet_pubkey: Pubkey,
    offering_token_mint: TokenMint,
    requesting_token_mint: TokenMint,
    original_offering_token_account: TokenAccount,
    original_requesting_token_account: TokenAccount,
    escrow: Empty[Escrow],
    escrow_offering_token_account: Empty[TokenAccount],
    escrow_requesting_token_account: Empty[TokenAccount],
):
    escrow = escrow.init(
        payer=offering_signer,
        seeds=[
            'escrow',
            original_offering_token_account,
            original_requesting_token_account
        ]
    )

    escrow.offering_wallet_pubkey = offering_signer.key()
    escrow.requesting_wallet_pubkey = requesting_wallet_pubkey

    escrow_offering_token_account = escrow_offering_token_account.init(
        payer=offering_signer,
        seeds=['escrow-offered-token-account',
               original_offering_token_account],
        mint=offering_token_mint,
        authority=escrow
    )

    escrow_requesting_token_account.init(
        payer=offering_signer,
        seeds=['escrow-requested-token-account',
               original_requesting_token_account],
        mint=requesting_token_mint,
        authority=escrow
    )

    escrow.offering_token_mint_pubkey = offering_token_mint.key()
    escrow.requesting_token_mint_pubkey = requesting_token_mint.key()

    escrow.offering_token_account_pubkey = escrow_offering_token_account.key()
    escrow.requesting_token_account_pubkey = escrow_requesting_token_account.key()

    assert offering_signer.key() == original_offering_token_account.authority(
    ), 'mismatch in token auth + signers'
    assert requesting_wallet_pubkey == original_requesting_token_account.authority(
    ), 'mismatch in token auth + requested pubkey'
    assert original_offering_token_account.amount == 1, 'the supply must equal 1 for the offered token'
    assert original_requesting_token_account.amount == 1, 'the supply must equal 1 for the requested token'


@instruction
def fund_escrow_offering_token_account(
    offering_signer: Signer,
    escrow: Escrow,
    original_offering_token_account: TokenAccount,
    escrow_offering_token_account: TokenAccount
):

    assert escrow.offering_wallet_pubkey == offering_signer.key(
    ), 'This swap escrow was not iniated by you.'
    assert escrow.offering_token_account_pubkey == escrow_offering_token_account.key(
    ), 'The escrow account does not match the given account.'
    assert escrow_offering_token_account.authority() == escrow.key(
    ), 'the given escrow_offering_token_account is now owned by the escrow'

    original_offering_token_account.transfer(
        offering_signer,
        escrow_offering_token_account,
        u64(1)
    )


@instruction
def defund_escrow_offering_token_account(
    offering_signer: Signer,
    escrow_bump: u8,
    escrow: Escrow,
    original_offering_token_account: TokenAccount,
    original_requesting_token_account: TokenAccount,
    escrow_offering_token_account: TokenAccount
):

    assert offering_signer.key(
    ) == escrow.offering_wallet_pubkey, 'This swap escrow was not iniated by you.'

    assert escrow.offering_token_account_pubkey == escrow_offering_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert escrow.requesting_token_account_pubkey == original_requesting_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert original_requesting_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'The escrow requested pubkey does not match the authority for the given token account'

    assert original_offering_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'The escrow offered pubkey does not match the authority for the given token account'

    escrow_offering_token_account.transfer(
        escrow,
        original_offering_token_account,
        u64(1),
        [
            'escrow',
            original_offering_token_account,
            original_requesting_token_account,
            escrow_bump
        ]
    )


@instruction
def fund_escrow_requesting_token_account(
    requesting_signer: Signer,
    escrow: Escrow,
    original_requesting_token_account: TokenAccount,
    escrow_requesting_token_account: TokenAccount
):

    assert escrow.requesting_wallet_pubkey == requesting_signer.key(
    ), 'This swap escrow was not requested to you.'

    assert escrow.requesting_token_account_pubkey == escrow_requesting_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert escrow_requesting_token_account.authority() == escrow.key(
    ), 'The given escrow_requesting_token_account is not owned by the escrow'

    original_requesting_token_account.transfer(
        requesting_signer,
        escrow_requesting_token_account,
        u64(1)
    )


@instruction
def defund_escrow_requesting_token_account(
    requesting_signer: Signer,
    escrow_bump: u8,
    escrow: Escrow,
    original_offering_token_account: TokenAccount,
    original_requesting_token_account: TokenAccount,
    escrow_requesting_token_account: TokenAccount
):

    assert escrow.requesting_wallet_pubkey == requesting_signer.key(
    ), 'This swap escrow was not requested to you.'

    assert escrow.requesting_token_account_pubkey == original_requesting_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert escrow.offering_token_account_pubkey == original_offering_token_account.key(
    ), 'The escrow account does not match the given account.'

    assert original_requesting_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'The escrow requested pubkey does not match the authority for the given token account'

    assert original_offering_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'The escrow offered pubkey does not match the authority for the given token account'

    escrow_requesting_token_account.transfer(
        escrow,
        original_requesting_token_account,
        u64(1),
        [
            'escrow',
            original_offering_token_account,
            original_requesting_token_account,
            escrow_bump
        ]
    )


@instruction
def crank_swap(
    escrow_bump: u8,
    escrow: Escrow,
    original_offering_token_account: TokenAccount,
    original_requesting_token_account: TokenAccount,
    escrow_offering_token_account: TokenAccount,
    escrow_requesting_token_account: TokenAccount,
    final_offering_token_account: TokenAccount,
    final_requesting_token_account: TokenAccount
):

    assert original_offering_token_account.authority() == final_requesting_token_account.authority(
    ), 'there is a mismatch in where the token should go'

    assert original_requesting_token_account.authority() == final_offering_token_account.authority(
    ), 'there is a mismatch in where the token should go'

    assert final_offering_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'the destination token account is now owned by the requested authority'

    assert final_requesting_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'the destination token account is now owned by the offering authority'

    assert escrow_offering_token_account.amount == 1, 'the escrow account does not have the offered token'

    assert escrow_requesting_token_account.amount == 1, 'the escrow account does not have the requested token'

    escrow_requesting_token_account.transfer(
        escrow,
        final_requesting_token_account,
        u64(1),
        [
            'escrow',
            original_offering_token_account,
            original_requesting_token_account,
            escrow_bump
        ]
    )

    escrow_offering_token_account.transfer(
        escrow,
        final_offering_token_account,
        u64(1),
        [
            'escrow',
            original_offering_token_account,
            original_requesting_token_account,
            escrow_bump
        ]
    )
