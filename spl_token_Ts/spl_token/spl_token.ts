// ts-node send_spl_token.ts

const {
    clusterApiUrl,
    Connection,
    Keypair,
    transaction,
    SystemProgram,
    LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

// https://solana-labs.github.io/solana-web3.js/

const  {
    createInitializeMintInstruction,
    TOKEN_PROGRAM_ID,
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    createMint,
}  = require("@solana/spl-token");
// https://solana-labs.github.io/solana-program-library/token/js/index.html

const bs58 = require("bs58");


// [168,143,20,100,123,221,21,179,250,115,88,102,185,163,95,68,220,240,164,33,81,219,228,32,230,148,248,242,81,66,141,15,192,188,222,195,54,31,59,227,235,79,109,149,202,41,216,173,205,218,117,228,34,46,63,43,26,144,218,152,175,197,132,55]

// DyNKK2N8HQVYfzFc6vQstFAuwabTtkgNewVG1GAQiuMg

const init = async () =>{
   
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let fromWallet = Keypair.generate();
    
      let fromAirdropSignature = await connection.requestAirdrop(
        fromWallet.publicKey,
        LAMPORTS_PER_SOL
      );

      await connection.confirmTransaction(fromAirdropSignature);

    let mintPubkey = await createMint(
        connection, // conneciton
        fromWallet, // fee payer
        fromWallet.publicKey, // mint authority
        fromWallet.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
        8 // decimals
      );

      console.log(`mint: ${mintPubkey.toBase58()}`);
    



    
}

init()


