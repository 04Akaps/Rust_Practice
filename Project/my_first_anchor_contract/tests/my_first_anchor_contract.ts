import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyFirstAnchorContract } from "../target/types/my_first_anchor_contract";

describe("my_first_anchor_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyFirstAnchorContract as Program<MyFirstAnchorContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
