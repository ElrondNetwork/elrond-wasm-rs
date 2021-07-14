elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm_derive::module]
pub trait EventsModule {
	#[event("buy-token")]
	fn buy_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

	#[event("sell-token")]
	fn sell_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}
