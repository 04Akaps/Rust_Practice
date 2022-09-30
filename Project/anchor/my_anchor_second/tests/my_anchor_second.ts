import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAnchorSecond } from "../target/types/my_anchor_second";

describe("my_anchor_second", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAnchorSecond as Program<MyAnchorSecond>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
