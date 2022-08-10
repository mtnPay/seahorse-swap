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
    ), 'The offering signing pubkey does not match the authority found on the given offering token account'
    assert requesting_wallet_pubkey == original_requesting_token_account.authority(
    ), 'The requesting pubkey does not match the authority found on the given requesting token account'
    assert original_offering_token_account.amount == 1, 'The given offering token account supply must be equal to 1'
    assert original_requesting_token_account.amount == 1, 'The given requesting token account supply must be equal to 1'


@instruction
def fund_escrow_offering_token_account(
    offering_signer: Signer,
    escrow: Escrow,
    original_offering_token_account: TokenAccount,
    escrow_offering_token_account: TokenAccount
):

    assert escrow.offering_wallet_pubkey == offering_signer.key(
    ), 'The offering signer pubkey did not match the offering wallet pubkey found on the given escrow'
    assert escrow.offering_token_account_pubkey == escrow_offering_token_account.key(
    ), 'The given escrow offering token account did not match the pubkey found on the given escrow'
    assert escrow_offering_token_account.authority() == escrow.key(
    ), 'The authority found on the given escrow offering token account did not match the pubkey from the given escrow'

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
    ) == escrow.offering_wallet_pubkey, 'The offering signer pubkey did not match the offering wallet pubkey on the given escrow'

    assert original_requesting_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'The authority on the given original reqesting token account did not match the requesting wallet pubkey found on the given escrow'

    assert original_offering_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'The authority on the given original offering token account did not match the offering wallet pubkey found on the given escrow'

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
    ), 'The requesting signer pubkey did not match the requesting wallet pubkey found on the given escrow'

    assert escrow.requesting_token_account_pubkey == escrow_requesting_token_account.key(
    ), 'The given escrow requesting token account did not match the pubkey found on the given escrow'

    assert escrow_requesting_token_account.authority() == escrow.key(
    ), 'The authority found on the given escrow requesting token account did not match the pubkey from the given escrow'

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
    ), 'The requesting signer pubkey did not match the requesting wallet pubkey on the given escrow'

    assert original_requesting_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'The authority on the given original reqesting token account did not match the requesting wallet pubkey found on the given escrow'

    assert original_offering_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'The authority on the given original offering token account did not match the offering wallet pubkey found on the given escrow'

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
    ), 'The authority on the original offering token account did not match the authoity on the final requesting token account'

    assert original_requesting_token_account.authority() == final_offering_token_account.authority(
    ), 'The authority on the requesting token account did not match the authority on the final offering token account'

    assert final_offering_token_account.authority(
    ) == escrow.requesting_wallet_pubkey, 'The authority on the final offering token account did not match the requesting wallet pubkey found on the given escrow'

    assert final_requesting_token_account.authority(
    ) == escrow.offering_wallet_pubkey, 'The authority on the final requesting token account did not match the offering wallet pubkey from the given escrow'

    assert escrow_offering_token_account.amount == 1, 'The amount of tokens in the escrow offering token account should equal 1'

    assert escrow_requesting_token_account.amount == 1, 'The amount of tokens in the escrow requesting token account should equal 1'

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
