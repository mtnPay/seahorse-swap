import * as anchor from "@project-serum/anchor"
import {
    MintLayout,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createInitializeMintInstruction,
    createMintToInstruction,
} from "@solana/spl-token"

export const mintNFT = async (
    minter: anchor.web3.Keypair,
    mint: anchor.web3.Keypair,
    connection: anchor.web3.Connection
) => {
    const userTokenAccountAddress = await findAssociatedTokenAddress(
        minter.publicKey,
        mint.publicKey
    )

    var createAccountInstruction = anchor.web3.SystemProgram.createAccount({
        fromPubkey: minter.publicKey,
        newAccountPubkey: mint.publicKey,
        space: MintLayout.span,
        lamports: await connection.getMinimumBalanceForRentExemption(
            MintLayout.span
        ),
        programId: TOKEN_PROGRAM_ID,
    })

    const initMint = createInitializeMintInstruction(
        mint.publicKey,
        0,
        minter.publicKey,
        minter.publicKey
    )

    const createAssociatedTokenAccount =
        createAssociatedTokenAccountInstruction(
            userTokenAccountAddress,
            minter.publicKey,
            minter.publicKey,
            mint.publicKey
        )

    const createMintTo = createMintToInstruction(
        mint.publicKey,
        userTokenAccountAddress,
        minter.publicKey,
        1,
        []
    )

    const blockhash = await connection.getLatestBlockhash()
    const transaction = new anchor.web3.Transaction({
        feePayer: minter.publicKey,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
    })
        .add(createAccountInstruction)
        .add(initMint)
        .add(createAssociatedTokenAccount)
        .add(createMintTo)

    const sig = await connection.sendTransaction(transaction, [minter, mint])

    await connection.confirmTransaction(sig)
}

export const createAssociatedTokenAccountInstruction = (
    associatedTokenAddress: anchor.web3.PublicKey,
    payer: anchor.web3.PublicKey,
    walletAddress: anchor.web3.PublicKey,
    splTokenMintAddress: anchor.web3.PublicKey
) => {
    const keys = [
        { pubkey: payer, isSigner: true, isWritable: true },
        { pubkey: associatedTokenAddress, isSigner: false, isWritable: true },
        { pubkey: walletAddress, isSigner: false, isWritable: false },
        { pubkey: splTokenMintAddress, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
            pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
            isSigner: false,
            isWritable: false,
        },
    ]
    return new anchor.web3.TransactionInstruction({
        keys,
        programId: ASSOCIATED_TOKEN_PROGRAM_ID,
        data: Buffer.from([]),
    })
}

export async function findAssociatedTokenAddress(
    walletAddress: anchor.web3.PublicKey,
    tokenMintAddress: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> {
    return (
        await anchor.web3.PublicKey.findProgramAddress(
            [
                walletAddress.toBuffer(),
                TOKEN_PROGRAM_ID.toBuffer(),
                tokenMintAddress.toBuffer(),
            ],
            ASSOCIATED_TOKEN_PROGRAM_ID
        )
    )[0]
}
