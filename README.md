# Voter Smart Contract

投票を行うことができるスマートコントラクトです。

Named keyとして、事前にALICE, BOB, CHARLIEの3人を作成しました。

## Rust toolchainのセットアップ
```bash
$ make prepare
```

## スマートコントラクトのコンパイル
```bash
$ make build-contract
```

## 全てを一括実行
```bash
$ make all
```

## 動作確認 (NCTL)

NCTL(casper-net-1)にスマートコントラクトをデプロイし、動作確認を行います。

### 1. スマートコントラクトのデプロイ

```bash
$ casper-client put-transaction session --node-address http://localhost:11101 --chain-name casper-net-1 --secret-key /casper-nctl/assets/net-1/faucet/secret_key.pem --payment-amount 50000000000 --wasm-path /voter/target/wasm32-unknown-unknown/release/voter-define.wasm --gas-price-tolerance 10 --install-upgrade  --session-entry-point call --standard-payment true
```

### 2. スマートコントラクトのデプロイが成功したかを確認

スマートコントラクトのデプロイが成功したかを確認し、後ほど使用するContract Hashを取得します。

```bash
casper-client get-transaction $TRANSACTION_HASH --n http://localhost:11101 
```

execution_infoの中の、"voter"という名前がついたkeyがContract Hashです。

![contract-hash](./image/contract-hash.png)

### 3. 票数のインクリメント

--session-argで"ALICE"を指定し、ALICEの票数をインクリメントします。

```bash
casper-client put-deploy --session-entry-point "voter_inc" --session-name "voter" --session-arg=candidate_name:"string='ALICE'" --payment-amount 5000000000 --chain-name casper-net-1 --n http://localhost:11101 --secret-key /casper-nctl/assets/net-1/faucet/secret_key.pem
```

### 4. 上記のput-deployが成功したかを確認

```bash
casper-client get-transaction $TRANSACTION_HASH --n http://localhost:11101 
```

### 5. State Root Hashの取得

後ほど必要となる、State Root Hashを取得します。

```bash
casper-client get-state-root-hash --n http://localhost:11101
```

### 6. 票数がインクリメントされていることを確認

ALICEの票数がインクリメントされていることを確認します。

```bash
casper-client query-state --n http://localhost:11101 -k <ContractHash> -s $STATE_ROOT_HASH -q ALICE | jq -r
```

