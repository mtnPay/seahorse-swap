import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow3 } from "../target/types/escrow3";

describe("escrow3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrow3 as Program<Escrow3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
