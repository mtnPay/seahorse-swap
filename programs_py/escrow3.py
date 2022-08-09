# escrow3
# Built with Seahorse v0.1.2

import string
from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Escrow(Account):
    offered_pubkey: Pubkey
    requested_pubkey: Pubkey
    offered_token_mint: TokenMint
    requested_token_mint: TokenMint
    offered_token_account: TokenAccount
    requested_token_account: TokenAccount


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
    # the new account that the requested token will be escrowed into
    new_requested_token_account: Empty[TokenAccount],
    # the base 58 pubkey string of the account that holds the token the initater wants to give
    offered_token_account_string: str,
    # the base 58 pubkey string of the account that holds the token the initater wants to receive
    requested_token_account_string: str,
):
    escrow = escrow.init(
        payer=offerer_signer,
        seeds=['escrow',
               offered_token_account_string,
               requested_token_account_string
               ]
    )
    escrow.offered_pubkey = offerer_signer.key()
    escrow.requested_pubkey = requested_pubkey
    escrow.offered_token_mint = offered_token_mint
    escrow.requested_token_mint = requested_token_mint

    new_offered_token_account.init(
        payer=offerer_signer,
        seeds=['token-account', escrow],
        mint=offered_token_mint,
        authority=escrow
    )
