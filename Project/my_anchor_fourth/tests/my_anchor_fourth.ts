import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAnchorFourth } from "../target/types/my_anchor_fourth";

describe("my_anchor_fourth", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAnchorFourth as Program<MyAnchorFourth>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
