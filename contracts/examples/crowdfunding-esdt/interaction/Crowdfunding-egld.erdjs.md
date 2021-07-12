# Crowdfunding ESDT - Using EGLD

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, Egld, wallets: { alice, bob, carol }} = await erdjs.setupInteractive("local-testnet");

let crowdfunding = await erdSys.loadWrapper("contracts/examples/crowdfunding-esdt");

// Set the deadline to 1 minute from now (adjust this if you want more time before claiming the rewards)
let someTimeFromNow = await erdSys.currentNonce() + erdjs.minutesToNonce(1);

// Deploy the crowdfunding contract with a target of 2 EGLD
await crowdfunding.sender(alice).gas(50_000_000).deploy(Egld(2), someTimeFromNow, Egld);

// Bob and carol contribute 1.5 EGLD each
await crowdfunding.sender(bob).gas(10_000_000).value(Egld(1.5)).call.fund();
await crowdfunding.sender(carol).value(Egld(1.5)).call.fund();

// Get the current funds. Note the usage of Egld.raw (since the balance comes as an integer from the smart contract)
let currentFunds = Egld.raw(await crowdfunding.query.currentFunds());

// Should print 3 EGLD (since bob and carol added 1.5 EGLD each)
erdjs.print(currentFunds);

// Confirming the target is 2 EGLD
erdjs.print(Egld.raw(await crowdfunding.query.get_target()));

// Check that alice is the owner
alice.address.equals(await crowdfunding.query.get_owner());

// Store alice's current balance (we'll use this to check the balance difference later on)
let aliceBalanceBefore = await erdSys.getBalance(alice, Egld);
erdjs.print(aliceBalanceBefore);

// Wait a minute first, otherwise you'll get the "cannot claim before deadline" error
// If the claim doesn't return an error - there are two possibilities:
// - the funding failed, and 1.5 EGLD are sent back to both bob and carol
// - it was succesful and alice receives 3 EGLD
// Because the target sum specified on deployment was 2 EGLD, and we have 3 EGLD, the funding should be succesful
await crowdfunding.sender(alice).call.claim();

// Let's check if alice received the funds
let aliceBalanceAfter = await erdSys.getBalance(alice, Egld);
erdjs.print(aliceBalanceAfter);

// If the previous claim was successful, this prints 2.99 EGLD (because of the gas costs)
erdjs.print(aliceBalanceAfter.minus(aliceBalanceBefore));
```
