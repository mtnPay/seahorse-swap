import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { Escrow3 } from "../target/types/escrow3"
import {
    MintLayout,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createInitializeMintInstruction,
    createMintToInstruction,
} from "@solana/spl-token"
import { min } from "bn.js"

describe("escrow3", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env())

    const program = anchor.workspace.Escrow3 as Program<Escrow3>

    const alice = anchor.web3.Keypair.generate()
    const bob = anchor.web3.Keypair.generate()

    let aliceMint: anchor.web3.PublicKey
    let bobMint: anchor.web3.PublicKey

    let aliceAta: anchor.web3.PublicKey
    let bobAta: anchor.web3.PublicKey

    let escrow: anchor.web3.PublicKey

    let offeredEscrowTokenAccount: anchor.web3.PublicKey
    let requestedEscrowTokenAccount: anchor.web3.PublicKey

    it("request airdrops", async () => {
        const atx = await program.provider.connection.requestAirdrop(
            alice.publicKey,
            1000000000
        )

        await program.provider.connection.confirmTransaction(atx)

        const btx = await program.provider.connection.requestAirdrop(
            bob.publicKey,
            1000000000
        )

        await program.provider.connection.confirmTransaction(btx)
    })

    it("mint NFT for alice", async () => {
        const mint = anchor.web3.Keypair.generate()
        await mintNFT(alice, mint, program.provider.connection)
        aliceMint = mint.publicKey
        aliceAta = await findAssociatedTokenAddress(alice.publicKey, aliceMint)
    })

    it("mint NFT for bob", async () => {
        const mint = anchor.web3.Keypair.generate()
        await mintNFT(bob, mint, program.provider.connection)
        bobMint = mint.publicKey
        bobAta = await findAssociatedTokenAddress(bob.publicKey, bobMint)
    })

    it("init escrow", async () => {
        escrow = await anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("escrow"), aliceAta.toBuffer(), bobAta.toBuffer()],
            program.programId
        )[0]

        offeredEscrowTokenAccount =
            await anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("escrow-offered-token-account"),
                    aliceAta.toBuffer(),
                ],
                program.programId
            )[0]

        requestedEscrowTokenAccount =
            await anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("escrow-requested-token-account"),
                    bobAta.toBuffer(),
                ],
                program.programId
            )[0]

        const ix = await program.methods
            .initEscrow(bobAta)
            .accounts({
                offererSigner: alice.publicKey,
                offeredTokenMint: aliceMint,
                requestedTokenMint: bobMint,
                offeredHolderTokenAccount: aliceAta,
                requestedHolderTokenAccount: bobAta,
                newOfferedTokenAccount: offeredEscrowTokenAccount,
                newRequestedTokenAccount: requestedEscrowTokenAccount,
                escrow: escrow,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: alice.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }).add(ix)

        const sig = await program.provider.connection.sendTransaction(tx, [
            alice,
        ])

        await program.provider.connection.confirmTransaction(sig)
    })

    it("fund escrow", async () => {
        const ix = await program.methods
            .fundEscrow()
            .accounts({
                offererSigner: alice.publicKey,
                offeredHolderTokenAccount: aliceAta,
                newOfferedTokenAccount: offeredEscrowTokenAccount,
                escrow: escrow,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: alice.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }).add(ix)

        const sig = await program.provider.connection.sendTransaction(tx, [
            alice,
        ])
    })
})

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

const mintNFT = async (
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
