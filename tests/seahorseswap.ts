import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { Seahorseswap } from "../target/types/seahorseswap"
import {
    mintNFT,
    findAssociatedTokenAddress,
    createAssociatedTokenAccountInstruction,
} from "./helpers"

const LAMPORTS_PER_SOL = 1_000_000_00

describe("seahorse-swap happy path", () => {
    anchor.setProvider(anchor.AnchorProvider.env())

    const program = anchor.workspace.Seahorseswap as Program<Seahorseswap>

    const alice = anchor.web3.Keypair.generate()
    const bob = anchor.web3.Keypair.generate()

    let aliceMint: anchor.web3.PublicKey
    let bobMint: anchor.web3.PublicKey

    let aliceAta: anchor.web3.PublicKey
    let bobAta: anchor.web3.PublicKey

    let escrow: anchor.web3.PublicKey
    let escrowBump: number

    let offeredEscrowTokenAccount: anchor.web3.PublicKey
    let requestedEscrowTokenAccount: anchor.web3.PublicKey

    it("request airdrops", async () => {
        const aliceAirdropTx = await program.provider.connection.requestAirdrop(
            alice.publicKey,
            LAMPORTS_PER_SOL
        )

        await program.provider.connection.confirmTransaction(aliceAirdropTx)

        const bobAirdropTx = await program.provider.connection.requestAirdrop(
            bob.publicKey,
            LAMPORTS_PER_SOL
        )

        await program.provider.connection.confirmTransaction(bobAirdropTx)
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

        const escrowPair = await anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("escrow"), aliceAta.toBuffer(), bobAta.toBuffer()],
            program.programId
        )

        escrow = escrowPair[0]
        escrowBump = escrowPair[1]

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
            .initEscrow(bob.publicKey)
            .accounts({
                offeringSigner: alice.publicKey,
                offeringTokenMint: aliceMint,
                requestingTokenMint: bobMint,
                originalOfferingTokenAccount: aliceAta,
                originalRequestingTokenAccount: bobAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
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

    it("fund offered token", async () => {
        const ix = await program.methods
            .fundEscrowOfferingTokenAccount()
            .accounts({
                offeringSigner: alice.publicKey,
                originalOfferingTokenAccount: aliceAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
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

    it("fund requested token", async () => {
        const ix = await program.methods
            .fundEscrowRequestingTokenAccount()
            .accounts({
                requestingSigner: bob.publicKey,
                originalRequestingTokenAccount: bobAta,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
                escrow: escrow,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: bob.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }).add(ix)

        const sig = await program.provider.connection.sendTransaction(tx, [bob])

        await program.provider.connection.confirmTransaction(sig)
    })

    // it("defund offered token", async () => {
    //     const ix = await program.methods
    //         .defundOfferedEscrow(escrowBump)
    //         .accounts({
    //             offeredSigner: alice.publicKey,
    //             escrow: escrow,
    //             offeredHolderTokenAccount: aliceAta,
    //             requestedHolderTokenAccount: bobAta,
    //             newOfferedTokenAccount: offeredEscrowTokenAccount,
    //         })
    //         .instruction()

    //     const blockhash = await program.provider.connection.getLatestBlockhash()
    //     var tx = new anchor.web3.Transaction({
    //         feePayer: alice.publicKey,
    //         blockhash: blockhash.blockhash,
    //         lastValidBlockHeight: blockhash.lastValidBlockHeight,
    //     }).add(ix)

    //     const sig = await program.provider.connection.sendTransaction(tx, [
    //         alice,
    //     ])

    //     await program.provider.connection.confirmTransaction(sig)
    // })

    // it("defund requested token", async () => {
    //     const ix = await program.methods
    //         .defundRequestedEscrow(escrowBump)
    //         .accounts({
    //             requestedSigner: bob.publicKey,
    //             escrow: escrow,
    //             offeredHolderTokenAccount: aliceAta,
    //             requestedHolderTokenAccount: bobAta,
    //             newRequestedTokenAccount: requestedEscrowTokenAccount,
    //         })
    //         .instruction()

    //     const blockhash = await program.provider.connection.getLatestBlockhash()
    //     var tx = new anchor.web3.Transaction({
    //         feePayer: bob.publicKey,
    //         blockhash: blockhash.blockhash,
    //         lastValidBlockHeight: blockhash.lastValidBlockHeight,
    //     }).add(ix)

    //     const sig = await program.provider.connection.sendTransaction(tx, [bob])

    //     await program.provider.connection.confirmTransaction(sig)
    // })

    it("crank swap", async () => {
        const finalAliceAta = await findAssociatedTokenAddress(
            alice.publicKey,
            bobMint
        )

        const finalBobAta = await findAssociatedTokenAddress(
            bob.publicKey,
            aliceMint
        )

        const aliceAtaInstructon =
            await createAssociatedTokenAccountInstruction(
                finalAliceAta,
                alice.publicKey,
                alice.publicKey,
                bobMint
            )

        const bobAtaInstructon = await createAssociatedTokenAccountInstruction(
            finalBobAta,
            alice.publicKey,
            bob.publicKey,
            aliceMint
        )

        const crankInsruction = await program.methods
            .crankSwap(escrowBump)
            .accounts({
                escrow: escrow,
                originalOfferingTokenAccount: aliceAta,
                originalRequestingTokenAccount: bobAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
                finalOfferingTokenAccount: finalBobAta,
                finalRequestingTokenAccount: finalAliceAta,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: alice.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        })
            .add(aliceAtaInstructon)
            .add(bobAtaInstructon)
            .add(crankInsruction)

        const sig = await program.provider.connection.sendTransaction(tx, [
            alice,
        ])

        await program.provider.connection.confirmTransaction(sig)
    })
})

describe("seahorse-swap defund", () => {
    anchor.setProvider(anchor.AnchorProvider.env())

    const program = anchor.workspace.Seahorseswap as Program<Seahorseswap>

    const alice = anchor.web3.Keypair.generate()
    const bob = anchor.web3.Keypair.generate()

    let aliceMint: anchor.web3.PublicKey
    let bobMint: anchor.web3.PublicKey

    let aliceAta: anchor.web3.PublicKey
    let bobAta: anchor.web3.PublicKey

    let escrow: anchor.web3.PublicKey
    let escrowBump: number

    let offeredEscrowTokenAccount: anchor.web3.PublicKey
    let requestedEscrowTokenAccount: anchor.web3.PublicKey

    it("request airdrops", async () => {
        const aliceAirdropTx = await program.provider.connection.requestAirdrop(
            alice.publicKey,
            LAMPORTS_PER_SOL
        )

        await program.provider.connection.confirmTransaction(aliceAirdropTx)

        const bobAirdropTx = await program.provider.connection.requestAirdrop(
            bob.publicKey,
            LAMPORTS_PER_SOL
        )

        await program.provider.connection.confirmTransaction(bobAirdropTx)
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

        const escrowPair = await anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("escrow"), aliceAta.toBuffer(), bobAta.toBuffer()],
            program.programId
        )

        escrow = escrowPair[0]
        escrowBump = escrowPair[1]

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
            .initEscrow(bob.publicKey)
            .accounts({
                offeringSigner: alice.publicKey,
                offeringTokenMint: aliceMint,
                requestingTokenMint: bobMint,
                originalOfferingTokenAccount: aliceAta,
                originalRequestingTokenAccount: bobAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
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

    it("fund offered token", async () => {
        const ix = await program.methods
            .fundEscrowOfferingTokenAccount()
            .accounts({
                offeringSigner: alice.publicKey,
                originalOfferingTokenAccount: aliceAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
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

    it("fund requested token", async () => {
        const ix = await program.methods
            .fundEscrowRequestingTokenAccount()
            .accounts({
                requestingSigner: bob.publicKey,
                originalRequestingTokenAccount: bobAta,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
                escrow: escrow,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: bob.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }).add(ix)

        const sig = await program.provider.connection.sendTransaction(tx, [bob])

        await program.provider.connection.confirmTransaction(sig)
    })

    it("defund offering token", async () => {
        const ix = await program.methods
            .defundEscrowOfferingTokenAccount(escrowBump)
            .accounts({
                offeringSigner: alice.publicKey,
                escrow: escrow,
                originalOfferingTokenAccount: aliceAta,
                originalRequestingTokenAccount: bobAta,
                escrowOfferingTokenAccount: offeredEscrowTokenAccount,
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

    it("defund requesting token", async () => {
        const ix = await program.methods
            .defundEscrowRequestingTokenAccount(escrowBump)
            .accounts({
                requestingSigner: bob.publicKey,
                escrow: escrow,
                originalOfferingTokenAccount: aliceAta,
                originalRequestingTokenAccount: bobAta,
                escrowRequestingTokenAccount: requestedEscrowTokenAccount,
            })
            .instruction()

        const blockhash = await program.provider.connection.getLatestBlockhash()
        var tx = new anchor.web3.Transaction({
            feePayer: bob.publicKey,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }).add(ix)

        const sig = await program.provider.connection.sendTransaction(tx, [bob])

        await program.provider.connection.confirmTransaction(sig)
    })
})
