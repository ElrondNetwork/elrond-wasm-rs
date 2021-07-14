elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::structs::{BondingCurve, Token};

#[elrond_wasm_derive::module]
pub trait StorageModule {
	fn get_token_nonce_ranges(&self, identifier: &TokenIdentifier) -> (EsdtTokenType, u64, u64) {
		let (token_type, _) = self.token_details(identifier).get();
		let mut min_loop_nonce = 0u64;
		let mut max_loop_nonce = 0u64;
		if token_type == EsdtTokenType::SemiFungible {
			min_loop_nonce = 1u64;
			max_loop_nonce = self
				.blockchain()
				.get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), identifier);
		}
		(token_type, min_loop_nonce, max_loop_nonce)
	}

	#[storage_mapper("token_details")]
	fn token_details(
		&self,
		token: &TokenIdentifier,
	) -> SingleValueMapper<Self::Storage, (EsdtTokenType, Address)>;

	#[storage_mapper("bonding_curve")]
	fn bonding_curve(
		&self,
		token: &Token,
	) -> SingleValueMapper<Self::Storage, BondingCurve<Self::BigUint>>;

	#[storage_mapper("owned_tokens")]
	fn owned_tokens(
		&self,
		owner: &Address,
	) -> SingleValueMapper<Self::Storage, Vec<TokenIdentifier>>;
}
