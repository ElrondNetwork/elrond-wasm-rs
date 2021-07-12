# Multisig

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, Egld, wallets: { alice, bob, carol }} = await erdjs.setupInteractive("local-testnet");

let multisig = await erdSys.loadWrapper("contracts/examples/multisig");

// Deploy a multisig contract with a quorum of 3, but 4 possible signers: alice, bob, carol, dan
// A quorum is the minimum required number of signers in order to be able to perform a proposed action
await multisig.sender(alice).gas(150_000_000).deploy(3, alice, bob, carol, dan);

// Deposit 10 EGLD from alice to the multisig contract, which we'll use them in the next examples
await multisig.gas(20_000_000).sender(alice).value(Egld(10)).deposit();

// Create a proposal to send 3 EGLD from the multisig to eve
// Note that any proposal is automatically signed by alice
var sendId = await multisig.sender(alice).proposeSendEgld(eve, Egld(3));

// Sign the previous proposal 2 more times
await multisig.sender(bob).sign(sendId);
await multisig.sender(carol).sign(sendId);

// This will return 3, which means that the quorum has been reached
await multisig.getActionValidSignerCount(sendId);

// Perform the send
await multisig.performAction(sendId);

// Let's use the adder contract as the nested contract which is to be managed by the multisig
let adder = await erdSys.loadWrapper("contracts/examples/adder");

// Validate and pack the arguments into a FormattedCall object
// Note: this doesn't deploy the contract (this will be done through the proposal below)
let formattedDeploy = adder.format.deploy(42);

// A proposal to deploy the adder smart contract. The formattedCall is automatically expanded.
var deployId = await multisig.sender(alice).gas(200_000_000).proposeSCDeploy(Egld(0), await adder.getCode(), false, false, false, formattedDeploy);

// Sign the deployment 2 more times
await multisig.sender(bob).gas(20_000_000).sign(deployId);
await multisig.sender(carol).sign(deployId);

// Perform the deploy. The address of the deployed adder will be returned.
var deployAddress = await multisig.performAction(deployId);

// Check the deploy address. bech32() will output it as erd1...
deployAddress.bech32();

// We can also access the adder smart contract by setting its deployed address to the smart contract wrapper instance.
// Let's check that the sum is 42 (the sum provided through the constructor).
await adder.address(deployAddress).sender(alice).getSum();

// Create a proposal on calling the "add" method
let formattedAdd = adder.format.add(1000);
var addId = await multisig.sender(alice).proposeSCCall(adder, Egld(0), formattedAdd);

// Sign the add proposal
await multisig.sender(bob).sign(addId);
await multisig.sender(carol).sign(addId);

// Before performing the "add" action, the sum should still be 42
await adder.sender(alice).getSum();

// Add 1000
await multisig.gas(40_000_000).performAction(addId);

// After adding, the sum should be 1042
await adder.sender(alice).getSum();
```
