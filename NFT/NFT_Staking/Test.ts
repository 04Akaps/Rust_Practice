import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
// https://www.npmjs.com/package/@metaplex-foundation/js

import {
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";

import { Provider, Program, setProvider, AnchorProvider, Wallet } from '@project-serum/anchor'

// https://coral-xyz.github.io/anchor/ts/index.html#setProvider

const programId = "BAnKeYtEAGTqg2TSzaUysFWFLGRExQ5gdqsF1zBW5zkS";
const id1 = require("./target/idl/basic_1.json")

const init  = async () =>{
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let fromWallet = Keypair.generate();

    console.log("")

    console.log("created Wallet is : ",fromWallet.publicKey.toBase58())
    
    console.log("")

    let fromAirdropSignature = await connection.requestAirdrop(
        fromWallet.publicKey,
        LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(fromAirdropSignature);


    const anchorWallet  = new Wallet(fromWallet);
    const provider = new AnchorProvider(connection, anchorWallet, {})


    const program = new Program(id1 ,programId, provider).methods

    console.log(program)

}

init()