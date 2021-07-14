use super::{big_uint_api_mock::*, RustBigInt};
use crate::async_data::AsyncCallTxData;
use crate::{SendBalance, TxContext, TxOutput, TxPanic};
use elrond_wasm::api::{BlockchainApi, ContractBase, SendApi, StorageReadApi, StorageWriteApi};
use elrond_wasm::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata, TokenIdentifier};
use num_bigint::BigUint;
use num_traits::Zero;

impl TxContext {
	fn get_available_egld_balance(&self) -> BigUint {
		// start with the pre-existing balance
		let mut available_balance = self.blockchain_info_box.contract_balance.clone();

		// add amount received
		available_balance += &self.tx_input_box.call_value;

		// already sent
		let tx_output = self.tx_output_cell.borrow();
		for send_balance in &tx_output.send_balance_list {
			available_balance -= &send_balance.amount;
		}

		available_balance
	}

	fn get_available_esdt_balance(&self, token_name: &[u8]) -> BigUint {
		// start with the pre-existing balance
		let mut available_balance = self
			.blockchain_info_box
			.contract_esdt
			.get(token_name)
			.unwrap_or(&BigUint::zero())
			.clone();

		// add amount received (if the same token)
		if self.tx_input_box.esdt_token_identifier == token_name {
			available_balance += &self.tx_input_box.esdt_value;
		}
		let tx_output = self.tx_output_cell.borrow();

		// already sent
		for send_balance in &tx_output.send_balance_list {
			if send_balance.token == token_name {
				available_balance -= &send_balance.amount;
			}
		}

		available_balance
	}
}

impl SendApi for TxContext {
	type AmountType = RustBigUint;
	type ProxyBigInt = RustBigInt;
	type ProxyStorage = Self;

	fn get_sc_address(&self) -> Address {
		BlockchainApi::get_sc_address(self)
	}

	fn get_gas_left(&self) -> u64 {
		BlockchainApi::get_gas_left(self)
	}

	fn direct_egld(&self, to: &Address, amount: &RustBigUint, _data: &[u8]) {
		if amount.value() > self.get_available_egld_balance() {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: b"failed transfer (insufficient funds)".to_vec(),
			});
		}

		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.send_balance_list.push(SendBalance {
			recipient: to.clone(),
			token: TokenIdentifier::egld(),
			amount: amount.value(),
		});
	}

	fn direct_egld_execute(
		&self,
		_to: &Address,
		_amount: &RustBigUint,
		_gas_limit: u64,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		panic!("direct_egld_execute not yet implemented")
	}

	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &RustBigUint,
		_gas: u64,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		if amount.value() > self.get_available_esdt_balance(token.as_esdt_identifier()) {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: b"insufficient funds".to_vec(),
			});
		}

		let mut tx_output = self.tx_output_cell.borrow_mut();
		tx_output.send_balance_list.push(SendBalance {
			recipient: to.clone(),
			token: token.clone(),
			amount: amount.value(),
		});
		Ok(())
	}

	fn direct_esdt_nft_execute(
		&self,
		_to: &Address,
		_token: &TokenIdentifier,
		_nonce: u64,
		_amount: &RustBigUint,
		_gas_limit: u64,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		panic!("direct_esdt_nft_execute not implemented yet");
	}

	fn async_call_raw(&self, to: &Address, amount: &RustBigUint, data: &[u8]) -> ! {
		// the cell is no longer needed, since we end in a panic
		let mut tx_output = self.tx_output_cell.replace(TxOutput::default());
		tx_output.async_call = Some(AsyncCallTxData {
			to: to.clone(),
			call_value: amount.value(),
			call_data: data.to_vec(),
			tx_hash: self.blockchain().get_tx_hash(),
		});
		std::panic::panic_any(tx_output)
	}

	fn deploy_contract(
		&self,
		_gas: u64,
		_amount: &RustBigUint,
		_code: &BoxedBytes,
		_code_metadata: CodeMetadata,
		_arg_buffer: &ArgBuffer,
	) -> Address {
		panic!("deploy_contract not yet implemented")
	}

	fn upgrade_contract(
		&self,
		_sc_address: &Address,
		_gas: u64,
		_amount: &RustBigUint,
		_code: &BoxedBytes,
		_code_metadata: CodeMetadata,
		_arg_buffer: &ArgBuffer,
	) {
		panic!("upgrade_contract not yet implemented")
	}

	fn execute_on_dest_context_raw(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes> {
		panic!("execute_on_dest_context_raw not implemented yet!");
	}

	fn execute_on_dest_context_raw_custom_result_range<F>(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
		_range_closure: F,
	) -> Vec<BoxedBytes>
	where
		F: FnOnce(usize, usize) -> (usize, usize),
	{
		panic!("execute_on_dest_context_raw_custom_result_range not implemented yet!");
	}

	fn execute_on_dest_context_by_caller_raw(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes> {
		panic!("execute_on_dest_context_by_caller_raw not implemented yet!");
	}

	fn execute_on_same_context_raw(
		&self,
		_gas: u64,
		_address: &Address,
		_value: &RustBigUint,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) {
		panic!("execute_on_same_context_raw not implemented yet!");
	}

	fn storage_store_tx_hash_key(&self, data: &[u8]) {
		let tx_hash = self.blockchain().get_tx_hash();
		self.storage_store_slice_u8(tx_hash.as_bytes(), data);
	}

	fn storage_load_tx_hash_key(&self) -> BoxedBytes {
		let tx_hash = self.blockchain().get_tx_hash();
		self.storage_load_boxed_bytes(tx_hash.as_bytes())
	}

	fn call_local_esdt_built_in_function(
		&self,
		_gas: u64,
		_function: &[u8],
		_arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes> {
		panic!("call_local_esdt_built_in_function not implemented yet!");
	}
}
