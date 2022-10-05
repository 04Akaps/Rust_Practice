
import {
    Connection,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL,
    Transaction,
    SystemProgram,
    PublicKey,
    sendAndConfirmTransaction
} from "@solana/web3.js";


import { Provider, Program, setProvider, AnchorProvider, Wallet, web3, BN} from '@project-serum/anchor'

import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";

import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";

const programId = "5YJa8BpFJPQuAh29pGgo4NYfYRqQ66m5QHj2p4o5SF1r";
const id1 = require("./target/idl/nft_staking.json")


const init = async () =>{
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

    const metaplex = new Metaplex(connection);
    metaplex.use(keypairIdentity(fromWallet));

    const beforeTokenAddress = new PublicKey("JAuDcoyGXdwSwjGQDBRq8NHuua6JCV3y7LDA7GDGC6jq");

    const mintNFTResponse = metaplex.nfts().create({
        payer : fromWallet, // 가스비 지급자
        updateAuthority : fromWallet, // nft를 바꿀 수 있는 wallet
        // updateAuthority는 mint를 해주는 계정으로 설정해 주어야 한다.
        
        mintAuthority : fromWallet, //  이전 민트 계정에 대한 mintrnjsgksdmf tjfwjd
        uri: "https://ffaaqinzhkt4ukhbohixfliubnvpjgyedi3f2iccrq4efh3s.arweave.net/KUAIIbk6p8oo4XHRcq0U__C2r0mwQaNl0gQow4Qp9yk", // 메타데이터 uri
        tokenOwner : fromWallet.publicKey, // token의 owner
        name:"hojin", // metadata의 이름
        sellerFeeBasisPoints : 3, // MetaPlexAuction을 통해서 거래되었을떄 받는 수수료 == > 3일경우 0.03%로 측정
        creators : [
            {
                address : fromWallet.publicKey,
                share : 100,
                authority : fromWallet
            }
        ],
        collection : beforeTokenAddress // 메타데이터에 추가되는 항목이로 collection으로 들어갑니다.
    });


    const nft = await mintNFTResponse.run();

    console.log("mint Address : ",nft.mintAddress.toBase58())

    // D2yH4MPakxoAZ9YJ3JRjTXdQFXzo3iDhJAcZkfLf4EDg

    const anchorWallet  = new Wallet(fromWallet);
    const provider = new AnchorProvider(connection, anchorWallet, {})
    const program = new Program(id1 ,programId, provider)


    const tx = await program.rpc.checkStakeNft({
        accounts: {
            store : fromWallet.publicKey,
            signer : anchorWallet.payer.publicKey,
            mint : nft.mintAddress.toBase58(),
            systemProgram : SystemProgram.programId.toBase58(),
            tokenProgram : TOKEN_PROGRAM_ID
        }
    })


    const afterData = await program.account.checkStore.fetch(fromWallet.publicKey)

    console.log(afterData.address.toBase58())

    // 9jvMyKScvbTHLThUqMhKTQ2EtXsjy7n1bzkBBWwkaUWx

}

init()