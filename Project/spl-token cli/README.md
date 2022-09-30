일단 token을 만들어 보자는 느낌으로 접근을 하였습니다.

기본적으로 spl-token-cli를 활용을 하였습니다.

- Sol Scan

https://solscan.io/token/FMuiXE3cn2pUMRMMSESuSNqReUKkkG3nnGF4kojSqowi?cluster=devnet#txs

간단하게 spl-token create를 하면 새로운 token주소가 만들어 집니다.

이떄 트랜잭션을 보내는 wallet은 solana config get에 설정된 주소가 됩니다.

이후 spl-token create-account를 입력하면 해당 프로그램에 주소를 하나 만들어 줍니다.

그후 spl-token mint <token address> <value>를 입력해주면 mint이 이루어 지는데 이떄 mint은 spl-token address에 입력이 됩니다.
**즉 누군가에게 mint하는 형태가 아니라 create-account해서 만든 계정에 mint가 됩니다..**
**일단 이 부분부터 제가 알고 있는 token mint시스템과 다릅니다.**
**또한 이후에 다시 create-account를 하고자 한다면 에러가 뜹니다..**

이후 Transfer를 하고자 한다면 spl-token transfer <token address> <value> <to>를 입력해 줍니다.

이떄 <to>주소는 해당 프로그램에 있는 account에게만 전송이 가능합니다.

하지만 실행을 하면 실질적으로 토큰이 주어지지 않을 것 입니다.

이는 해당 to주소에 token Account가 없기 떄문입니다.

https://solscan.io/account/H9gLdCx7ka4SoQpdHXPMNqw16TihhRPvn7dT6aa7QcZ2?cluster=devnet

해당 링크를 보면 알 수 있듯이 program마다 특정 wallet에 연결된 token-account가 존재합니다.

제가 가지고 있는 wallet의 주소는 5XoFbNzkv2ku1P5K2rfnbBTErXQVn5y7ybY5J13hZVLe이고,
token의 address는 FMuiXE3cn2pUMRMMSESuSNqReUKkkG3nnGF4kojSqowi 입니다.

이러한 정보를 토대로 해당 Scan을 살펴보면 연동된 Token-wallet이라는 것을 알 수 있습니다.

그러면 저희는 다른 계정을 만들고 해당 계정에서 Token프로그램과 연동되는 associated_address를 만들어 준 뒤에 해당 계정으로 mint 및 transfer하면 됩니다.

solana config get을 통해서 현재 사용중인 id를 찾고 해당 id.json파일을 따로 보관합니다.
**이를 1번 계정이라고 하겠습니다.**

이후 solana-keygen new를 통해서 계정을 새로 만들어 주고 이후 해당 계정또한 따로 보관합니다.
**이는 2번 계정입니다.**

이후 2번 계정을 통해서 spl-token create-account <token address>를 넣어줌으로써 token프로그램에 연동된 토큰 지갑을 만들어 줍니다.

그 후 1번 계정으로 다시 연결한 뒤에 spl-token mint <token address> <value> <2번 계정의 associated token address>를 입력해 주면 mint가 이루어 집니다.
**계정연결은 id.json파일을 교체해주면 됩니다.**

Transfer또한 동일하게 작동합니다.

```
이 사실에서 알 수 있는것은

Solana는 특이하게도 Token program마다 연동된 계정을 만들고 해당 계정에 balance를 보관한다는 점 입니다.
```
