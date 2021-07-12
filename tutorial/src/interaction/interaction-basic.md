# Preparations

# Setting up the local testnet

The following examples rely on having a [local testnet](https://docs.elrond.com/developers/setup-local-testnet/) up and running.

# Installing @elrondnetwork/erdjs globally

```bash
cd ./code/elrond-sdk-erdjs
npm run compile && npm test && npm install -g
```

# How to start a node terminal

By exporting `NODE_PATH`, the node terminal should have access to `erdjs`.
Open a terminal and enter the following:

```bash
cd ./code/elrond-wasm-rs
export NODE_PATH=$HOME/.nvm/versions/node/$(node --version)/lib/node_modules
node --experimental-repl-await
```

# Basic ESDT usage

- [Fungible Tokens (FTs)](esdt-FT-fungible-tokens.md)
- [Semi-Fungible Tokens (SFTs)](esdt-SFT-semi-fungible-tokens.md)
- [Non-Fungible Tokens (NFTs)](esdt-NFT-non-fungible-tokens.md)

# Smart contract examples

- Adder [interaction](../../../contracts/examples/adder/interaction/Adder.erdjs.md)
- Crowdfunding ESDT [EGLD interaction](../../../contracts/examples/crowdfunding-esdt/interaction/Crowdfunding-egld.erdjs.md), [ESDT interaction](../../../contracts/examples/crowdfunding-esdt/interaction/Crowdfunding-esdt.erdjs.md)
- Multisig [EGLD adder interaction](../../../contracts/examples/multisig/interaction/Multisig-adder-egld.erdjs.md)
- Ping-pong [EGLD interaction](../../../contracts/examples/ping-pong-egld/interaction/Ping-pong-egld.erdjs.md)
