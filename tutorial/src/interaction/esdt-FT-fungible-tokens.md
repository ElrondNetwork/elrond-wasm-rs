# Fungible Tokens (FT)

Fungible Tokens have variable amounts, but always have nonce 0. They may be denominated.

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, wallets: { alice, bob, carol } } = await erdjs.setupInteractive("local-testnet");

// Issue a new fungible token
let MyToken = await erdSys.sender(alice).issueFungible("MyFungibleToken", "MYTOKEN", 1_000_00, 2);

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

// Note: if you have the token identifier, you can recall the token via:
// let MyToken = await erdSys.recallToken("MYTOKEN-a4fc62");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await erdSys.getBalance(alice, MyToken).then(erdjs.print);

// Send some tokens to bob
await erdSys.sender(alice).value(MyToken(200.0)).send(bob);

// Check alice's balance (should be 800.00 MYTOKEN)
await erdSys.getBalance(alice, MyToken).then(erdjs.print);

// Check bob's balance (should be 200.00 MYTOKEN)
await erdSys.getBalance(bob, MyToken).then(erdjs.print);

```
