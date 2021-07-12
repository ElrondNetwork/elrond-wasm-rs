# Ping-pong

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, Egld, wallets: { alice, bob, carol, dan } } = await erdjs.setupInteractive("local-testnet");

let pingPong = await erdSys.loadWrapper("contracts/examples/ping-pong-egld");

await pingPong.sender(alice).gas(150_000_000).deploy(Egld(0.5), 2 * 60, null, Egld(1.5));

await pingPong.gas(20_000_000).sender(alice).value(Egld(0.5)).ping("note 1");

await pingPong.sender(bob).value(Egld(0.5)).ping(null);
await pingPong.sender(carol).value(Egld(0.5)).ping(null);

// this fails because of the balance limit of 1.5 egld
await pingPong.sender(dan).value(Egld(0.5).ping(null);

await pingPong.pongAll();

```
