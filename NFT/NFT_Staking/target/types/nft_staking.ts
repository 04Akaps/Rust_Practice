export type NftStaking = {
  "version": "0.1.0",
  "name": "nft_staking",
  "instructions": [
    {
      "name": "transfer",
      "accounts": [
        {
          "name": "storage",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NotEnoughTime",
      "msg": "not enough time to reclaim"
    },
    {
      "code": 6001,
      "name": "NoMatchMetadata",
      "msg": "invalid metadata information"
    },
    {
      "code": 6002,
      "name": "NoMatchSymbol",
      "msg": "invalid token"
    }
  ]
};

export const IDL: NftStaking = {
  "version": "0.1.0",
  "name": "nft_staking",
  "instructions": [
    {
      "name": "transfer",
      "accounts": [
        {
          "name": "storage",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NotEnoughTime",
      "msg": "not enough time to reclaim"
    },
    {
      "code": 6001,
      "name": "NoMatchMetadata",
      "msg": "invalid metadata information"
    },
    {
      "code": 6002,
      "name": "NoMatchSymbol",
      "msg": "invalid token"
    }
  ]
};
