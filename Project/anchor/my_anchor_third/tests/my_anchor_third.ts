import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAnchorThird } from "../target/types/my_anchor_third";

describe("my_anchor_third", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAnchorThird as Program<MyAnchorThird>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
