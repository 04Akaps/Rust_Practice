import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
// https://www.npmjs.com/package/@metaplex-foundation/js

import {
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction,
} from "@solana/web3.js";

import { Provider, Program, setProvider, AnchorProvider, Wallet, web3, BN,} from '@project-serum/anchor'

const { SystemProgram } = web3;
// https://coral-xyz.github.io/anchor/ts/index.html#setProvider

// DyNKK2N8HQVYfzFc6vQstFAuwabTtkgNewVG1GAQiuMg

const programId = "BPrW9qJNafGsKTWraRecHapUXxQs8hbrR6VMDoLicfbL";
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


    // ** anchor 설정 **

    const anchorWallet  = new Wallet(fromWallet);
    const provider = new AnchorProvider(connection, anchorWallet, {})
    const program = new Program(id1 ,programId, provider)

    const newTestAccount = web3.Keypair.generate();


    // const tx = await program.rpc.initialize(new BN(3), {
    //   accounts : {
    //     myAccount : newTestAccount.publicKey,
    //     user : anchorWallet.payer.publicKey,
    //     systemProgram : SystemProgram.programId.toBase58()
    //   },
    //   signers : [newTestAccount]
    // })

    // console.log(tx)

}

init()
