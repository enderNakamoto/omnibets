# Overview

### ⚠️⚠️⚠️ Caution! This is beta / testnet technology ⚠️⚠️⚠️

## WIP - TODO

-   better documentation for contract to contract example
-   document Rust contract for contract to contract example

# Introduction

NEAR's MPC allows a NEAR Account to create derivative accounts (public keys) and signatures of transactions for other blockchains.

Several MPC nodes maintain a single public key. This key is combined with your NEAR AccountId (unique) and a chosen "path" offset (chosen by client). This produces a new and unique public key. The generation of signatures via the MPC nodes can only be authorized by same NEAR Account by calling the `sign` method of the MPC contract.

The creation of secp256k1 public keys for Bitcoin and EVM chains is currently supported.

### Flow (how it works)

1. Obtain the MPC public key (near view [MPC_CONTRACT_ID] `public_key`) and hardcode into `.env` or code
2. Choose a path for the derived account (public key) see: [Path naming conventions](https://github.com/near/near-fastauth-wallet/blob/dmd/chain_sig_docs/docs/chain_signature_api.org)
3. Use `./src/kdf.ts -> generateAddress` to generate the derived account address and public key
4. Use the `sign` method of `./src/near.ts -> sign` which calls the MPC contract to sign payload (hash of TX)
5. Using a library (ethers/bitcoinjs-lib) combine the transaction and signature to create signed transaction
6. Broadcast the transaction e.g. `sendRawTransaction`

# Installation

`yarn`

### CREATE .env FILE in root of project

```
NEAR_ACCOUNT_ID="[NEAR_TESTNET_ACCOUNT]"
NEAR_PRIVATE_KEY="[NEAR_ACCOUNT_PRIVATE_KEY]"
MPC_PATH="[MPC_PATH]"
MPC_CHAIN="[ethereum|bitcoin]"
MPC_CONTRACT_ID="multichain-testnet-2.testnet"
MPC_PUBLIC_KEY="secp256k1:4HFcTSodRLVCGNVcGc4Mf2fwBBBxv9jxkGdiW2S2CA1y6UpVVRWKj6RX7d7TDt65k2Bj3w9FU4BGtt43ZvuhCnNt"
```

### For dogecoin testnet (link below)

```
TATUM_API_KEY=""
```

### For MPC_PATH please refer to:

[Path naming conventions](https://github.com/near/near-fastauth-wallet/blob/dmd/chain_sig_docs/docs/chain_signature_api.org)

# How to Use Commands

(as a user or dev to verify everything works)

1. Read the `Installation` steps and set up all environment variables first with `.env` file.
2. Use the commands to generate addresses first.
3. Fund these addresses with the Testnet Faucets provided in the links below.
4. Use the commands to send funds from your generated addresses.

# Prebuilt Commands

`yarn start [commands]`

### Command List

-   -ea - ethereum addressm (EVM)
-   -ba - bitcoin testnet address
-   -da - dogecoin testnet address
-   -ra - ripple testnet address
-   -s - sign sample payload using NEAR account
-   -etx - send ETH
-   -btx - send BTC
-   -dtx - send DOGE (requires API KEY)
-   -rtx - send XRP

### Sending Options

-   --amount - amount to send (ETH or sats)
-   --to - destination address

# EVM Contract Deployment and Interactions (advanced)

Usage: `yarn start [commands]`

### Command List

-   -d, -edc - deploy contract
-   --to - the contract address to view/call
-   -v, -view - view contract state (readonly call)
-   -c, -call - call contract method
-   --path - path to EVM bytecode file from root of this project
-   --method - name of method view/call
-   --args - arguments e.g. '{"address":"0x525521d79134822a342d330bd91da67976569af1"}' in single quotes
-   --ret - list of return parameter types (if any) e.g. ['uint256']

## Ethereum EVM Contract NFT Example

After setting up all your environment variables and ensuring your calling EVM address has ETH for gas.

Start by deploying a new NFT contract:

1. `yarn start -d`

Check explorer link and make sure contract is deployed successfully.

Take your contract address from console result and call:

2. `yarn start -c --to 0x[CONTRACT ADDRESS FROM STEP 1]`

This will mint a token to default address `0x525521d79134822a342d330bd91da67976569af1`.

To mint a token to a different address use `--args '{"address":"0x[SOME_OTHER_ADDRESS]"}'` with your args in single quotes and properly formatted JSON paths and values in double quotes.

View the balance of the **default address** using:

3. `yarn start -v --to 0x[CONTRACT ADDRESS FROM STEP 1]`

Which should output `1` the NFT balance of default address `0x525521d79134822a342d330bd91da67976569af1`

To view the balance of a different address use `--args '{"address":"0x[SOME_OTHER_ADDRESS]"}'` with your args in single quotes and properly formatted JSON paths and values in double quotes.

# Proxy call MPC sign from NEAR Contract and use signature to call 'mint' on EVM Contract (advanced)

To deploy the NEAR contract use `cargo-near`.

Install `cargo-near` and `near-cli`

-   [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
-   [near CLI-rs](https://near.cli.rs) - Iteract with NEAR blockchain from command line

```
cargo build

cargo near create-dev-account

cargo near deploy [ACCOUNT_ID]
```

The NEAR contract has the following features:

1. `sign` method accepts a payload that is the unhashed RLP encoded EVM transaction data e.g. `6a627842000000000000000000000000525521d79134822a342d330bd91DA67976569aF1` calls the method `mint` with an address argument of `525521d79134822a342d330bd91DA67976569aF1`
2. `PUBLIC_RLP_ENCODED_METHOD_NAMES` stores public EVM method name hashes that can be called from this NEAR contract to the destination address e.g. the method name `mint` hashes `6a627842000000000000000000000000`
3. `COST` must be paid in NEAR
4. `path` and `key_version` arguments are passed through to MPC `sign` call, but in the future could be used as additional features for applications or security

To use, set the following `.env` vars accordingly:

```
NEAR_PROXY_ACCOUNT="true"
NEAR_PROXY_CONTRACT="true"
NEAR_PROXY_ACCOUNT_ID="futuristic-anger.testnet"
NEAR_PROXY_PRIVATE_KEY="ed25519:..."
```

With `NEAR_PROXY_CONTRACT="true"` the script will call `sign` method of the proxy contract you deployed using `cargo near deploy`.

To verify, send some ETH using `yarn start -etx`.

With `NEAR_PROXY_ACCOUNT="false"` you will not be able to send ETH using the `sign` method of the proxy contract. Why? Because this would mean any NEAR account can send ETH from the derived account of the proxy contract. Oh no! The proxy contract protects against arbitrary transactions using this check:

```
let owner = env::predecessor_account_id() == env::current_account_id();

// check if rlp encoded eth transaction is calling a public method name
let mut public = false;
for n in PUBLIC_RLP_ENCODED_METHOD_NAMES {
	if rlp_payload.find(n).is_some() {
		public = true
	}
}

// only the NEAR contract owner can call sign of arbitrary payloads for chain signature accounts based on env::current_account_id()
if !public {
	require!(
		owner,
		"only contract owner can sign arbitrary EVM transactions"
	);
}
```

## Testing the Proxy call and minting the NFT on the EVM Contract

1. Your contract should be deployed and you should have the following env vars:

NEAR_PROXY_ACCOUNT="true"
NEAR_PROXY_CONTRACT="true"
NEAR_PROXY_ACCOUNT_ID="..."
NEAR_PROXY_PRIVATE_KEY="ed25519:..."

2. Call `yarn start -etx` you are now deriving an Ethereum address using the NEAR Account ID of the NEAR Proxy Contract, not your NEAR Account ID

This Ethereum address is different and unfunded. So, this transaction will not work.

3. Fund the address from step 2. You can do this by sending ETH using this script.

Change env vars:

NEAR_PROXY_ACCOUNT="false"
NEAR_PROXY_CONTRACT="false"

Send ETH to your new derived Ethereum address:

`yarn start -etx --to 0x[ADDRESS FROM STEP 2] --amount [AMOUNT IN ETH e.g. 0.1]`

4. Now you can repeat the steps from **Ethereum EVM Contract NFT Example** above.

## Key Points

1. To use the NEAR Contract, when you are not the NEAR Contract owner, the NEAR transaction requires `1 NEAR` token
2. The Ethereum transaction always uses gas from the same account derived from = NEAR_PROXY_ACCOUNT_ID + MPC_PATH (+ MPC_PUBLIC_KEY)

Enjoy!

# References & Useful Links

### Examples

[Live Example - NEAR Testnet, Sepolia, Bitcoin Testnet](https://test.near.social/md1.testnet/widget/chainsig-sign-eth-tx)

[A frontend example you can run locally](https://github.com/gagdiez/near-multichain)

### Docs

[Path naming conventions](https://github.com/near/near-fastauth-wallet/blob/dmd/chain_sig_docs/docs/chain_signature_api.org)

[Chain Signatures Docs](https://docs.near.org/concepts/abstraction/chain-signatures)

[Chain Signatures Use Cases](https://docs.near.org/concepts/abstraction/signatures/use-cases)

### MPC Repositories

[MPC Repo](https://github.com/near/mpc-recovery/)

### Faucets and API Keys

[Sepolia Faucet](https://sepolia-faucet.pk910.de/)

[Sepolia Faucet](https://cloud.google.com/application/web3/faucet/ethereum/sepolia)

[Bitcoin Testnet Faucet](https://faucet.triangleplatform.com/bitcoin/testnet)

#### For Dogecoin, you will need to register for Tatum API (free plan):

[Dogecoin Tatum API](https://tatum.io/) and [docs](https://apidoc.tatum.io/tag/Dogecoin)

[Dogecoin Testnet Faucet](https://shibe.technology/)

#### XRP Ledger

[XRP Ledger Testnet Faucet](https://test.bithomp.com/faucet/)

[XRP Ledger Testnet Explorer](https://test.bithomp.com/explorer)
