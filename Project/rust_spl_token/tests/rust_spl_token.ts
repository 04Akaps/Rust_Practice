import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RustSplToken } from "../target/types/rust_spl_token";

describe("rust_spl_token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RustSplToken as Program<RustSplToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
