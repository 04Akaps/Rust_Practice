NFT를 Scan에 Deploy하는 것은 문제가 아닌거 같습니다.

그런데 좀더 기능적으로 추가해야합니다.

만약 NFT를 Mint하는 비용을 1 SOL 이라고 해서 이를 Owner가 받아야 한다면 ...??

EVM에서는 간단하게 msg.value를 통해서 설정하면 되지만 Sol에서는 따로 Contract를 작성한 것이 아니기 떄문에 이러한 기능을 제공하는지 한번 알아봐야 겠습니다.

이후 발행한 NFT를 어떻게 다른 Contract에서 또 활용하고 할지가 문제기도 하고요...

1. Mint할떄마다 새로운 Token Address가 부여된다.

2. Mint Authority는 자동으로 Metaplex로 잡히게 된다.

3. 그러기 떄문에 Update Authority만 특정 회사 Owner계정으로 해두면 된다.
