# Non-Fungible Tokens (NFTs)

Non-Fungible Tokens have amounts of either 0 or 1, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, Egld, wallets: { alice, bob, carol } } = await erdjs.setupInteractive("local-testnet");

// Issue a new non-fungible token
let MyToken = await erdSys.sender(alice).issueNonFungible("MyFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await erdSys.esdtSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "ESDTRoleNFTCreate");

// Create 2 tokens
let MyFirstNFT = await erdSys.sender(alice).esdtNftCreate(MyToken, 1, "MyFirstNFT", 0, "", "", "https://example.com");
let MySecondNFT = await erdSys.sender(alice).esdtNftCreate(MyToken, 1, "MySecondNFT", 0, "", "", "https://example.com");

// Send some tokens to bob and carol
await erdSys.sender(alice).value(MyFirstNFT.one()).send(bob);
await erdSys.sender(alice).value(MySecondNFT.one()).send(carol);

await erdSys.getBalanceList(alice, MyToken).then(erdjs.printList);
await erdSys.getBalanceList(bob, MyToken).then(erdjs.printList);
```
