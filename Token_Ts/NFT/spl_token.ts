import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
// https://www.npmjs.com/package/@metaplex-foundation/js

import {
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";


const init = async () => {
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


    const mintNFTResponse = metaplex.nfts().create({
        payer : fromWallet, // 가스비 지급자
        updateAuthority : fromWallet, // nft를 바꿀 수 있는 wallet
        mintAuthority : fromWallet, //  이전 민트 계정에 대한 mintrnjsgksdmf tjfwjd
        uri: "https://ffaaqinzhkt4ukhbohixfliubnvpjgyedi3f2iccrq4efh3s.arweave.net/KUAIIbk6p8oo4XHRcq0U__C2r0mwQaNl0gQow4Qp9yk", // 메타데이터 uri
        tokenOwner : fromWallet.publicKey, // token의 owner
        name:"hojin", // metadata의 이름
        sellerFeeBasisPoints : 3, // MetaPlexAuction을 통해서 거래되었을떄 받는 수수료 == > 3일경우 0.03%로 측정
    });


    const nft = await mintNFTResponse.run();

    console.log(nft.mintAddress.toBase58())


    // 94oLMWPMJ2vasqZBiU2RBTcEe6kEjz7bTBhVPTABeNAU
    // -> minted Address

    // 아래있는 주석은 create메서드의 타입 및 변수

    //   export declare type CreateNftInput = {

    //     payer?: Signer;
    //     updateAuthority?: Signer;
    //     mintAuthority?: Signer;
    //     useNewMint?: Signer;
    //     useExistingMint?: PublicKey;
    //     tokenOwner?: PublicKey;
    //     tokenAddress?: PublicKey | Signer;

    //     uri: string;
    //     name: string;
    //     sellerFeeBasisPoints: number;

    //     symbol?: string;
    //     creators?: CreatorInput[];
    //     isMutable?: boolean;
    //     maxSupply?: Option<BigNumber>;
    //     uses?: Option<Uses>;
    //     isCollection?: boolean;
    //     collection?: Option<PublicKey>;
    //     collectionAuthority?: Option<Signer>;
    //     collectionAuthorityIsDelegated?: boolean;
    //     collectionIsSized?: boolean;
    //     tokenProgram?: PublicKey;
    //     associatedTokenProgram?: PublicKey;
    //     confirmOptions?: ConfirmOptions;
    // };
}

init()