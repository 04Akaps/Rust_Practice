// 배포한 rust contract를 이제 sdk를 통해서 소통하는 부분 입니다.
// 기본적으로 target폴더의 idl을 ABI 및 address값으로 인식하여 활용하였습니다.

// #[account]
// pub struct MyAccount {
//     pub data : u64,
// }

const jsonData = require("./target/idl/basic_1.json");

const { Connection, PublicKey } = require("@solana/web3.js");
const { Provider, Program, web3 } = require("@project-serum/anchor");

const connection = new Connection("https://api.devnet.solana.com", "processed");

const program = new Program(
  jsonData,
  jsonData.metadata.address,
  "5XoFbNzkv2ku1P5K2rfnbBTErXQVn5y7ybY5J13hZVLe"
);

const init = async () => {
  await program.methods.initialize(10, {});
};

// pub fn initialize(ctx :Context<Initialize> , data : u64) -> Result<()> {
//     let my_account = &mut ctx.accounts.my_account;
//     my_account.data = data;
//     Ok(())
// }

init();
