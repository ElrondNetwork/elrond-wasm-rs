use super::{ArwenBigInt, ArwenBigUint, ArwenEllipticCurve};
use crate::ArwenApiImpl;
use alloc::vec::Vec;
use elrond_wasm::api::{BlockchainApi, SendApi, StorageReadApi, StorageWriteApi};
use elrond_wasm::types::{Address, ArgBuffer, BoxedBytes, CodeMetadata, TokenIdentifier};

extern "C" {
	fn transferValue(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		dataLength: i32,
	) -> i32;

	fn transferValueExecute(
		dstOffset: *const u8,
		valueOffset: *const u8,
		gasLimit: i64,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn transferESDTExecute(
		dstOffset: *const u8,
		tokenIdOffset: *const u8,
		tokenIdLen: i32,
		valueOffset: *const u8,
		gasLimit: i64,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn transferESDTNFTExecute(
		dstOffset: *const u8,
		tokenIdOffset: *const u8,
		tokenIdLen: i32,
		valueOffset: *const u8,
		nonce: i64,
		gasLimit: i64,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn asyncCall(
		dstOffset: *const u8,
		valueOffset: *const u8,
		dataOffset: *const u8,
		length: i32,
	) -> !;

	fn createContract(
		gas: i64,
		valueOffset: *const u8,
		codeOffset: *const u8,
		codeMetadataOffset: *const u8,
		codeLength: i32,
		resultOffset: *const u8,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn upgradeContract(
		scAddressOffset: *const u8,
		gas: i64,
		valueOffset: *const u8,
		codeOffset: *const u8,
		codeMetadataOffset: *const u8,
		codeLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	);

	fn executeOnDestContext(
		gas: i64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn executeOnDestContextByCaller(
		gas: i64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn executeOnSameContext(
		gas: i64,
		addressOffset: *const u8,
		valueOffset: *const u8,
		functionOffset: *const u8,
		functionLength: i32,
		numArguments: i32,
		argumentsLengthOffset: *const u8,
		dataOffset: *const u8,
	) -> i32;

	fn getNumReturnData() -> i32;
	fn getReturnDataSize(result_index: i32) -> i32;
	fn getReturnData(result_index: i32, dataOffset: *const u8) -> i32;
}

impl SendApi for ArwenApiImpl {
	type AmountType = ArwenBigUint;
	type ProxyBigInt = ArwenBigInt;
	type ProxyEllipticCurve = ArwenEllipticCurve;
	type ProxyStorage = Self;

	#[inline]
	fn get_sc_address(&self) -> Address {
		BlockchainApi::get_sc_address(self)
	}

	#[inline]
	fn get_gas_left(&self) -> u64 {
		BlockchainApi::get_gas_left(self)
	}

	fn direct_egld(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = transferValue(
				to.as_ref().as_ptr(),
				amount_bytes32_ptr,
				data.as_ptr(),
				data.len() as i32,
			);
		}
	}

	fn direct_egld_execute(
		&self,
		to: &Address,
		amount: &ArwenBigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let result = transferValueExecute(
				to.as_ref().as_ptr(),
				amount_bytes32_ptr,
				gas_limit as i64,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
			if result == 0 {
				Ok(())
			} else {
				Err(b"transferValueExecute failed")
			}
		}
	}

	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &ArwenBigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let result = transferESDTExecute(
				to.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				amount_bytes32_ptr,
				gas_limit as i64,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
			if result == 0 {
				Ok(())
			} else {
				Err(b"transferESDTExecute failed")
			}
		}
	}

	fn direct_esdt_nft_execute(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &ArwenBigUint,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]> {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let result = transferESDTNFTExecute(
				to.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				amount_bytes32_ptr,
				nonce as i64,
				gas_limit as i64,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
			if result == 0 {
				Ok(())
			} else {
				Err(b"transferESDTNFTExecute failed")
			}
		}
	}

	fn async_call_raw(&self, to: &Address, amount: &ArwenBigUint, data: &[u8]) -> ! {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			asyncCall(
				to.as_ref().as_ptr(),
				amount_bytes32_ptr,
				data.as_ptr(),
				data.len() as i32,
			)
		}
	}

	fn deploy_contract(
		&self,
		gas: u64,
		amount: &ArwenBigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address {
		let mut new_address = Address::zero();
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = createContract(
				gas as i64,
				amount_bytes32_ptr,
				code.as_ptr(),
				code_metadata.as_ptr(),
				code.len() as i32,
				new_address.as_mut_ptr(),
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
		new_address
	}

	fn upgrade_contract(
		&self,
		sc_address: &Address,
		gas: u64,
		amount: &ArwenBigUint,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			upgradeContract(
				sc_address.as_ref().as_ptr(),
				gas as i64,
				amount_bytes32_ptr,
				code.as_ptr(),
				code_metadata.as_ptr(),
				code.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn execute_on_dest_context_raw(
		&self,
		gas: u64,
		address: &Address,
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes> {
		unsafe {
			let num_return_data_before = getNumReturnData();

			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = executeOnDestContext(
				gas as i64,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);

			let num_return_data_after = getNumReturnData();
			get_return_data_range(num_return_data_before, num_return_data_after)
		}
	}

	fn execute_on_dest_context_raw_custom_result_range<F>(
		&self,
		gas: u64,
		address: &Address,
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
		range_closure: F,
	) -> Vec<BoxedBytes>
	where
		F: FnOnce(usize, usize) -> (usize, usize),
	{
		unsafe {
			let num_return_data_before = getNumReturnData();

			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = executeOnDestContext(
				gas as i64,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);

			let num_return_data_after = getNumReturnData();
			let (result_start_index, result_end_index) = range_closure(
				num_return_data_before as usize,
				num_return_data_after as usize,
			);
			get_return_data_range(result_start_index as i32, result_end_index as i32)
		}
	}

	fn execute_on_dest_context_by_caller_raw(
		&self,
		gas: u64,
		address: &Address,
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes> {
		unsafe {
			let num_return_data_before = getNumReturnData();

			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = executeOnDestContextByCaller(
				gas as i64,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);

			let num_return_data_after = getNumReturnData();
			get_return_data_range(num_return_data_before, num_return_data_after)
		}
	}

	fn execute_on_same_context_raw(
		&self,
		gas: u64,
		address: &Address,
		amount: &ArwenBigUint,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) {
		unsafe {
			let amount_bytes32_ptr = amount.unsafe_buffer_load_be_pad_right(32);
			let _ = executeOnSameContext(
				gas as i64,
				address.as_ref().as_ptr(),
				amount_bytes32_ptr,
				function.as_ptr(),
				function.len() as i32,
				arg_buffer.num_args() as i32,
				arg_buffer.arg_lengths_bytes_ptr(),
				arg_buffer.arg_data_ptr(),
			);
		}
	}

	fn storage_store_tx_hash_key(&self, data: &[u8]) {
		let tx_hash = self.get_tx_hash();
		self.storage_store_slice_u8(tx_hash.as_bytes(), data);
	}

	fn storage_load_tx_hash_key(&self) -> BoxedBytes {
		let tx_hash = self.get_tx_hash();
		self.storage_load_boxed_bytes(tx_hash.as_bytes())
	}

	fn call_local_esdt_built_in_function(&self, gas: u64, function: &[u8], arg_buffer: &ArgBuffer) {
		// account-level built-in function, so the destination address is the contract itself
		let own_address = BlockchainApi::get_sc_address(self);

		let _ = self.execute_on_dest_context_raw(
			gas,
			&own_address,
			&ArwenBigUint::from(0u32),
			function,
			arg_buffer,
		);
	}
}

/// Retrieves already pushed results, via `finish`.
/// `from_index` is inclusive.
/// `to_index` is exclusive.
unsafe fn get_return_data_range(from_index: i32, to_index: i32) -> Vec<BoxedBytes> {
	let num_results = to_index - from_index;
	let mut result = Vec::with_capacity(num_results as usize);
	if num_results > 0 {
		for index in from_index..to_index {
			result.push(get_return_data(index));
		}
	}
	result
}

/// Retrieves already pushed individual result at given index, via `finish`.
unsafe fn get_return_data(return_index: i32) -> BoxedBytes {
	let len = getReturnDataSize(return_index);
	let mut res = BoxedBytes::allocate(len as usize);
	if len > 0 {
		let _ = getReturnData(return_index, res.as_mut_ptr());
	}
	res
}
