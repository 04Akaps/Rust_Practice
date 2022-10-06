import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
// https://www.npmjs.com/package/@metaplex-foundation/js

import {
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
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

    console.log("NFT Address : ",nft.mintAddress.toBase58())


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

    // export declare type CreatorInput = {
    //     /** The public key of the creator. */
    //     readonly address: PublicKey;
    //     /** The creator's shares of the royalties in percent (i.e. 5 is 5%). */
    //     readonly share: number;
    //     /**
    //      * The authority that should sign the asset to verify the creator.
    //      * When this is not provided, the creator will not be
    //      * verified within this operation.
    //      */
    //     readonly authority?: Signer;
    // };


    const largestAccounts = await connection.getTokenLargestAccounts(
        new PublicKey(nft.mintAddress.toBase58())
    );
    // 해당 NFT에 대한 수량과 잡다한 값들이 나온다.
    console.log("")
    console.log("")

    const largestAccountInfo = await connection.getParsedAccountInfo(
        largestAccounts.value[0].address
    );

    console.log("Token Program : ",largestAccountInfo.value?.owner.toBase58())
    // NFT에 대한 OwnerProgram == Token program이 나온다.

    console.log("NFT Owner : ",largestAccountInfo.value?.data)
    // 이 부분을 확인하면 NFT Owner항목이 보이게 된다.

    console.log("")
    console.log("")

    // 계정이 가지고 있는 전체 NFT가져오기

    const owner = new PublicKey(fromWallet.publicKey.toBase58())

    const allNFTs = metaplex.nfts().findAllByOwner({owner : owner});
    // docs에서는 이렇게 끝낱지만 사실상 타입이 조금 다름
    // -> 그러기 떄문에 이렇게 수정

    // export declare type FindNftsByOwnerInput = {
    //     /** The address of the owner. */
    //     owner: PublicKey;
    //     /** The level of commitment desired when querying the blockchain. */
    //     commitment?: Commitment;
    // };

    const getNFT = await allNFTs.run();
    // 또한 Pending처리가 뜨고 있기 떄문에 해당 값을 가져오기 까지 비동기도 동작시킴으로써 값을 올바르게 가져온다.

    // @ts-ignore
    // @ts-nocheck
    console.log(getNFT[0].mintAddress.toBase58())
    // 이상하게 해당 키값이 있지만 보이지 않는다고 컴파일에서 에러가 뜨기 떄문에 주석으로 코드체크를 무시해 줍니다.
    // 그러면 원하는 NFT Address를 가져 올 수 있습니다.

    //   const largestAccountInfo = await connection.getParsedAccountInfo(
    //     largestAccounts.value[0].address
    //   );
}

init()