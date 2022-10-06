
import {
    Connection,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL,
    SystemProgram,
    PublicKey,
} from "@solana/web3.js";

import {  Program,  AnchorProvider, Wallet, web3} from '@project-serum/anchor'

import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";

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

    console.log("")
    console.log("")

    console.log("airdrop success!!")

    console.log("")
    console.log("")

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

    setTimeout(async () => {
        console.log("맛좀 보자!!")
        const anchorWallet  = new Wallet(fromWallet);
        const provider = new AnchorProvider(connection, anchorWallet, {})
        const program = new Program(id1 ,programId, provider)

        const [escrowPDA]  = await web3.PublicKey.findProgramAddress([
            utf8.encode('test')
        ], program.programId)


    const tx = await program.rpc.transfer({
        accounts: {
            // storage : escrowPDA,
            to : nft.mintAddress.toBase58(),
            signer : fromWallet.publicKey,
            mint : nft.mintAddress.toBase58(),
            systemProgram : SystemProgram.programId.toBase58(),
            tokenProgram : TOKEN_PROGRAM_ID
        }
    })

    console.log(tx)
    }, 5000)

    
}

init()