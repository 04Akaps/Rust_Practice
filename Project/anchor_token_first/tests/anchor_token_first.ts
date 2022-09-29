import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorTokenFirst } from "../target/types/anchor_token_first";

describe("anchor_token_first", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorTokenFirst as Program<AnchorTokenFirst>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
