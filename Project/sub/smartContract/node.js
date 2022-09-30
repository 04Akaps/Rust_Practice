const { struct, u32, ns64 } = require("@solana/buffer-layout");
const { Buffer } = require("buffer");
const web3 = require("@solana/web3.js");

const init = async () => {
  // let keypair = web3.Keypair.generate();
  let payer = web3.Keypair.generate();

  let connection = new web3.Connection(
    web3.clusterApiUrl("devnet"),
    "confirmed"
  );

  // await connection.getBalance(payer.publicKey).then((result) => {
  //   console.log(result);
  // });

  let airdropSignature = await connection.requestAirdrop(
    payer.publicKey,
    web3.LAMPORTS_PER_SOL
  );
  let airdropSignature2 = await connection.requestAirdrop(
    keypair.publicKey,
    web3.LAMPORTS_PER_SOL
  );

  await connection.confirmTransaction(airdropSignature);
  // await connection.confirmTransaction(airdropSignature2);

  // await connection.getBalance(payer.publicKey).then((result) => {
  //   console.log(result);
  // });

  let allocateTransaction = new web3.Transaction({
    feePayer: payer.publicKey,
  });
  let keys = [{ pubkey: keypair.publicKey, isSigner: true, isWritable: true }];
  let params = { space: 100 };

  let allocateStruct = {
    index: 8,
    layout: struct([u32("instruction"), ns64("space")]),
  };

  let data = Buffer.alloc(allocateStruct.layout.span);
  let layoutFields = Object.assign(
    { instruction: allocateStruct.index },
    params
  );
  allocateStruct.layout.encode(layoutFields, data);

  allocateTransaction.add(
    new web3.TransactionInstruction({
      keys,
      programId: "2JJxXu1WnT5BEkWMb2CKAv6ZtM4XtjgypiqcTYqSKFME",
      data,
    })
  );

  await web3.sendAndConfirmTransaction(connection, allocateTransaction, [
    payer,
    keypair,
  ]);
};

init();
