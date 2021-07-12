# Semi-Fungible Tokens (SFTs).

Semi-Fungible Tokens have variable amounts, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, Egld, wallets: { alice, bob, carol } } = await erdjs.setupInteractive("local-testnet");

// Issue a new semi-fungible token
let MyToken = await erdSys.sender(alice).issueSemiFungible("MySemiFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await erdSys.esdtSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "ESDTRoleNFTCreate", "ESDTRoleNFTAddQuantity");

// Create a new nonce
let MyFirstSemi = await erdSys.sender(alice).esdtNftCreate(MyToken, 1_000, "MyFirstSemi", 0, "", "", "https://example.com");

// Check alice's token balance
// Note: if the balance comes up as 0, wait some time and try again
await erdSys.getBalance(alice, MyFirstSemi).then(erdjs.print);

// Send some tokens to bob and carol
await erdSys.sender(alice).value(MyFirstSemi(200)).send(bob);

await erdSys.getBalance(alice, MyFirstSemi).then(erdjs.print);
await erdSys.getBalance(bob, MyFirstSemi).then(erdjs.print);

```
