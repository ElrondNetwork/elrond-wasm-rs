use super::managed_types::RustBigUint;
use crate::TxContext;
use elrond_wasm::{
	api::BigUintApi,
	types::{Address, EsdtTokenData, TokenIdentifier, H256},
};

impl elrond_wasm::api::BlockchainApi for TxContext {
	type BalanceType = RustBigUint;

	fn get_sc_address(&self) -> Address {
		self.tx_input_box.to.clone()
	}

	fn get_owner_address(&self) -> Address {
		self.blockchain_info_box
			.contract_owner
			.clone()
			.unwrap_or_else(|| panic!("contract owner address not set"))
	}

	fn get_shard_of_address(&self, _address: &Address) -> u32 {
		panic!("get_shard_of_address not implemented")
	}

	fn is_smart_contract(&self, _address: &Address) -> bool {
		panic!("is_smart_contract not implemented")

		/*
		Mock used when testing the marketplace contract

		let mut addr_slice = [0u8; 32];
		hex::decode_to_slice(b"6d61726b6574706c6163655f636f6e74726163745f5f5f5f5f5f5f5f5f5f5f5f",
			&mut addr_slice);

		_address == &Address::from_slice(&addr_slice)
		*/
	}

	fn get_caller(&self) -> Address {
		self.tx_input_box.from.clone()
	}

	fn get_balance(&self, address: &Address) -> RustBigUint {
		if address != &self.get_sc_address() {
			panic!("get balance not yet implemented for accounts other than the contract itself");
		}
		self.blockchain_info_box.contract_balance.clone().into()
	}

	fn get_tx_hash(&self) -> H256 {
		self.tx_input_box.tx_hash.clone()
	}

	fn get_gas_left(&self) -> u64 {
		self.tx_input_box.gas_limit
	}

	fn get_block_timestamp(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_timestamp
	}

	fn get_block_nonce(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_nonce
	}

	fn get_block_round(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_round
	}

	fn get_block_epoch(&self) -> u64 {
		self.blockchain_info_box.current_block_info.block_epoch
	}

	fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		self.blockchain_info_box
			.current_block_info
			.block_random_seed
			.clone()
	}

	fn get_prev_block_timestamp(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_timestamp
	}

	fn get_prev_block_nonce(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_nonce
	}

	fn get_prev_block_round(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_round
	}

	fn get_prev_block_epoch(&self) -> u64 {
		self.blockchain_info_box.previous_block_info.block_epoch
	}

	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		self.blockchain_info_box
			.previous_block_info
			.block_random_seed
			.clone()
	}

	fn get_current_esdt_nft_nonce(&self, _address: &Address, _token: &TokenIdentifier) -> u64 {
		// TODO: Implement
		0u64
	}

	// TODO: Include nonce and create a map like: TokenId -> Nonce -> Amount
	fn get_esdt_balance(
		&self,
		address: &Address,
		token: &TokenIdentifier,
		_nonce: u64,
	) -> RustBigUint {
		if address != &self.get_sc_address() {
			panic!(
				"get_esdt_balance not yet implemented for accounts other than the contract itself"
			);
		}

		match self
			.blockchain_info_box
			.contract_esdt
			.get(&token.as_esdt_identifier().to_vec())
		{
			Some(value) => value.clone().into(),
			None => RustBigUint::zero(),
		}
	}

	fn get_esdt_token_data(
		&self,
		_address: &Address,
		_token: &TokenIdentifier,
		_nonce: u64,
	) -> EsdtTokenData<RustBigUint> {
		panic!("get_esdt_token_data not yet implemented")
	}
}
