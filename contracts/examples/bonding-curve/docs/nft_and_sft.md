# NFTs and SFTs
On the create function we create the nonce, reason why for NFT and SFT would be the best place to store the details about the `supply` and the `accepted_token`
```rust
fn create(
	&self,
	identifier: TokenIdentifier,
	amount: Self::BigUint,
	name: BoxedBytes,
	royalties: Self::BigUint,
	hash: BoxedBytes,
	attributes: BoxedBytes,
	uri: BoxedBytes,
	#[var_args] max_supply: OptionalArg<Self::BigUint>,
	#[var_args] supply_type: OptionalArg<SupplyType>,
	#[var_args] payment: OptionalArg<TokenIdentifier>,
) -> SCResult<()> 
```
# Behaviour
- SFT: 
  - you pass through this function once per nonce, so the **optional arguments** should always be provided
- NFT: 
  - first time you pass through this function you should provide the optional arguments for setting them in the storage 
  -  from 2nd time further the **optional arguments** are ignored.
