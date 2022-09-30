import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAnchor } from "../target/types/my_anchor";

describe("my_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAnchor as Program<MyAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
