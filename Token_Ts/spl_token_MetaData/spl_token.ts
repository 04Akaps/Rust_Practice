// ts-node send_spl_token.ts

import {
    clusterApiUrl,
    Connection,
    Keypair,
    SystemProgram,
    LAMPORTS_PER_SOL,
    PublicKey,
    sendAndConfirmTransaction
}  from "@solana/web3.js"

// https://solana-labs.github.io/solana-web3.js/


import { 
    createMint} from "@solana/spl-token"
// https://solana-labs.github.io/solana-program-library/token/js/index.html

import bs58 from "bs58"

import * as mpl from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@project-serum/anchor";

const init = async () =>{
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let fromWallet = Keypair.generate();

    console.log("")

    console.log("created Wallet is : ",fromWallet)
    
    console.log("")

    console.log("created Wallet Public Key is : ",fromWallet.publicKey.toBase58())

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
        8, // decimals
    );

    console.log(`Token Contract Address : ${mintPubkey.toBase58()}`);

    const mint = new PublicKey(mintPubkey);
    // 새로운 publicket 객체를 만들어 줍니다.

    const seed1 = Buffer.from(anchor.utils.bytes.utf8.encode("metadata"));
    const seed2 = Buffer.from(mpl.PROGRAM_ID.toBytes());
    const seed3 = Buffer.from(mint.toBytes());


    const [metadataPDA, _bump] = PublicKey.findProgramAddressSync([seed1, seed2, seed3], mpl.PROGRAM_ID);

    // console.log(metadataPDA.toBase58())
    // Account로 뜬다

    // https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#findProgramAddressSync
    // docs를 읽어보면 시드와 결합하면 유효한 프로그램 주소가 될떄까지 반복하고 값을 내놓는다 라고 적혀잇는데
    // 아직은 잘 이해가 되지가 않습니다..

    const accounts = {
        metadata :metadataPDA,
        mint : mint,
        mintAuthority : fromWallet.publicKey,
        payer : fromWallet.publicKey,
        updateAuthority : fromWallet.publicKey,
    }

    const metadata = {
        name : "Love Me Funk",
        symbol : "USD",
        uri : "https://images.deepai.org/machine-learning-models/ca6bd68b90a64e75b2e195434bab73d3/biggan_demo_copy.jpg",
        sellerFeeBasisPoints : 0,
        creators : null, 
        collection : null,
        uses : null
    }

//     export declare type DataV2 = {
//         name: string;
//         symbol: string;
//         uri: string;
//         sellerFeeBasisPoints: number;
//         creators: beet.COption<Creator[]>;
//         collection: beet.COption<Collection>;
//         uses: beet.COption<Uses>;
//     };

//     export declare type CreateMetadataAccountArgsV2 = {
//         data: DataV2;
//         isMutable: boolean;
//     };


// export declare function createCreateMetadataAccountV2Instruction(accounts: CreateMetadataAccountV2InstructionAccounts, args: CreateMetadataAccountV2InstructionArgs, programId?: web3.PublicKey): web3.TransactionInstruction;

// 타입이 맞지 않기 떄문에 모듈을 뜯어 가면서 타입을 대조 및 오타 수정


    const args = {
        createMetadataAccountArgsV2 : {
            data  : metadata,
            isMutable : true
        }
    }

    const ix = mpl.createCreateMetadataAccountV2Instruction(accounts, args);


    const tx = new anchor.web3.Transaction();
    tx.add(ix);

    const txid = await sendAndConfirmTransaction(connection, tx, [fromWallet])
    console.log(txid)

    
    
}

init()


