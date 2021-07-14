use elrond_codec::{TopDecode, TopEncode};

use super::{BigIntApi, BigUintApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::{
	types::{Address, ArgBuffer, AsyncCall, BoxedBytes, CodeMetadata, TokenIdentifier, Vec},
	HexCallDataSerializer,
};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApi: ErrorApi + Clone + Sized {
	/// The type of the payment arguments.
	/// Not named `BigUint` to avoid name collisions in types that implement multiple API traits.
	type AmountType: BigUintApi + 'static;

	/// Not used by `SendApi`, but forwarded to the proxy traits.
	type ProxyBigInt: BigIntApi + 'static;

	/// Not used by `SendApi`, but forwarded to the proxy traits.
	type ProxyStorage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Required for ESDTNFTTransfer.
	/// Same as the implementation from BlockchainApi.
	fn get_sc_address(&self) -> Address;

	/// To be used internally by the SendApi implementation.
	/// Do not use directly from contracts. It might be removed from this trait at some point or reworked.
	fn get_gas_left(&self) -> u64;

	/// Sends EGLD to a given address, directly.
	/// Used especially for sending EGLD to regular accounts.
	fn direct_egld(&self, to: &Address, amount: &Self::AmountType, data: &[u8]);

	/// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
	fn direct_egld_execute(
		&self,
		to: &Address,
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends ESDT to an address and executes like an async call, but without callback.
	fn direct_esdt_execute(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends ESDT NFT to an address and executes like an async call, but without callback.
	#[allow(clippy::too_many_arguments)]
	fn direct_esdt_nft_execute(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &Self::AmountType,
		gas_limit: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Result<(), &'static [u8]>;

	/// Sends either EGLD, ESDT or NFT to the target address,
	/// depending on the token identifier and nonce
	fn direct(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &Self::AmountType,
		data: &[u8],
	) {
		if token.is_egld() {
			self.direct_egld(to, amount, data);
		} else if nonce == 0 {
			let _ = self.direct_esdt_execute(to, token, amount, 0, data, &ArgBuffer::new());
		} else {
			let _ =
				self.direct_esdt_nft_execute(to, token, nonce, amount, 0, data, &ArgBuffer::new());
		}
	}

	/// Sends an asynchronous call to another contract.
	/// Calling this method immediately terminates tx execution.
	/// Using it directly is generally discouraged.
	///
	/// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
	/// Use a `HexCallDataSerializer` to prepare this field.
	fn async_call_raw(&self, to: &Address, amount: &Self::AmountType, data: &[u8]) -> !;

	/// Sends an asynchronous call to another contract, with either EGLD or ESDT value.
	/// The `token` argument decides which one it will be.
	/// Calling this method immediately terminates tx execution.
	fn async_call(&self, async_call: AsyncCall<Self>) -> ! {
		self.async_call_raw(
			&async_call.to,
			&async_call.egld_payment,
			async_call.hex_data.as_slice(),
		)
	}

	/// Performs a simple ESDT/NFT transfer, but via async call.  
	/// As with any async call, this immediately terminates the execution of the current call.  
	/// So only use as the last call in your endpoint.  
	/// If you want to perform multiple transfers, use `self.send().direct()` instead.  
	/// Note that EGLD can NOT be transfered with this function.  
	fn transfer_esdt_via_async_call(
		&self,
		to: &Address,
		token: &TokenIdentifier,
		nonce: u64,
		amount: &Self::AmountType,
		data: &[u8],
	) -> ! {
		if nonce == 0 {
			let mut serializer = HexCallDataSerializer::new(ESDT_TRANSFER_STRING);
			serializer.push_argument_bytes(token.as_esdt_identifier());
			serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
			if !data.is_empty() {
				serializer.push_argument_bytes(data);
			}

			self.async_call_raw(to, &Self::AmountType::zero(), serializer.as_slice())
		} else {
			let mut serializer = HexCallDataSerializer::new(ESDT_NFT_TRANSFER_STRING);
			serializer.push_argument_bytes(token.as_esdt_identifier());
			serializer.push_argument_bytes(&nonce.to_be_bytes()[..]);
			serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
			serializer.push_argument_bytes(to.as_bytes());
			if !data.is_empty() {
				serializer.push_argument_bytes(data);
			}

			self.async_call_raw(
				&self.get_sc_address(),
				&Self::AmountType::zero(),
				serializer.as_slice(),
			);
		}
	}

	/// Deploys a new contract in the same shard.
	/// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
	/// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
	fn deploy_contract(
		&self,
		gas: u64,
		amount: &Self::AmountType,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	) -> Address;

	/// Upgrades a child contract of the currently executing contract.
	/// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
	/// The child contract's new init function will be called with the provided arguments
	fn upgrade_contract(
		&self,
		sc_address: &Address,
		gas: u64,
		amount: &Self::AmountType,
		code: &BoxedBytes,
		code_metadata: CodeMetadata,
		arg_buffer: &ArgBuffer,
	);

	fn change_owner_address(&self, child_sc_address: &Address, new_owner: &Address) {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(new_owner.as_bytes());

		let _ = self.execute_on_dest_context_raw(
			self.get_gas_left(),
			child_sc_address,
			&Self::AmountType::zero(),
			b"ChangeOwnerAddress",
			&arg_buffer,
		);
	}

	/// Same shard, in-line execution of another contract.
	fn execute_on_dest_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	/// Same shard, in-line execution of another contract.
	/// Allows the contract to specify which result range to extract as sync call result.
	/// This is a workaround to handle nested sync calls.
	/// Please do not use this method unless there is absolutely no other option.
	/// Will be eliminated after some future Arwen hook redesign.
	/// `range_closure` takes the number of results before, the number of results after,
	/// and is expected to return the start index (inclusive) and end index (exclusive).
	fn execute_on_dest_context_raw_custom_result_range<F>(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
		range_closure: F,
	) -> Vec<BoxedBytes>
	where
		F: FnOnce(usize, usize) -> (usize, usize);

	fn execute_on_dest_context_by_caller_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	fn execute_on_same_context_raw(
		&self,
		gas: u64,
		address: &Address,
		value: &Self::AmountType,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	);

	/// Used to store data between async call and callback.
	fn storage_store_tx_hash_key(&self, data: &[u8]);

	/// Used to store data between async call and callback.
	fn storage_load_tx_hash_key(&self) -> BoxedBytes;

	/// Allows synchronously calling a local function by name. Execution is resumed afterwards.
	/// You should never have to call this function directly.
	/// Use the other specific methods instead.
	fn call_local_esdt_built_in_function(
		&self,
		gas: u64,
		function: &[u8],
		arg_buffer: &ArgBuffer,
	) -> Vec<BoxedBytes>;

	/// Allows synchronous minting of ESDT/SFT (depending on nonce). Execution is resumed afterwards.
	/// Note that the SC must have the ESDTLocalMint or ESDTNftAddQuantity roles set,
	/// or this will fail with "action is not allowed"
	/// For SFTs, you must use `self.send().esdt_nft_create()` before adding additional quantity.
	/// This function cannot be used for NFTs.
	fn esdt_local_mint(&self, token: &TokenIdentifier, nonce: u64, amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		let func_name: &[u8];

		arg_buffer.push_argument_bytes(token.as_esdt_identifier());

		if nonce == 0 {
			func_name = b"ESDTLocalMint";
		} else {
			func_name = b"ESDTNFTAddQuantity";
			arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		}

		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		let _ = self.call_local_esdt_built_in_function(self.get_gas_left(), func_name, &arg_buffer);
	}

	/// Allows synchronous burning of ESDT/SFT/NFT (depending on nonce). Execution is resumed afterwards.
	/// Note that the SC must have the ESDTLocalBurn or ESDTNftBurn roles set,
	/// or this will fail with "action is not allowed"
	fn esdt_local_burn(&self, token: &TokenIdentifier, nonce: u64, amount: &Self::AmountType) {
		let mut arg_buffer = ArgBuffer::new();
		let func_name: &[u8];

		arg_buffer.push_argument_bytes(token.as_esdt_identifier());

		if nonce == 0 {
			func_name = b"ESDTLocalBurn";
		} else {
			func_name = b"ESDTNFTBurn";
			arg_buffer.push_argument_bytes(&nonce.to_be_bytes()[..]);
		}

		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());

		let _ = self.call_local_esdt_built_in_function(self.get_gas_left(), func_name, &arg_buffer);
	}

	/// Creates a new NFT token of a certain type (determined by `token_identifier`).  
	/// `attributes` can be any serializable custom struct.  
	/// This is a built-in function, so the smart contract execution is resumed after.
	/// Must have ESDTNftCreate role set, or this will fail with "action is not allowed".
	/// Returns the nonce of the newly created NFT.
	#[allow(clippy::too_many_arguments)]
	fn esdt_nft_create<T: elrond_codec::TopEncode>(
		&self,
		token: &TokenIdentifier,
		amount: &Self::AmountType,
		name: &BoxedBytes,
		royalties: &Self::AmountType,
		hash: &BoxedBytes,
		attributes: &T,
		uris: &[BoxedBytes],
	) -> u64 {
		let mut arg_buffer = ArgBuffer::new();
		arg_buffer.push_argument_bytes(token.as_esdt_identifier());
		arg_buffer.push_argument_bytes(amount.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(name.as_slice());
		arg_buffer.push_argument_bytes(royalties.to_bytes_be().as_slice());
		arg_buffer.push_argument_bytes(hash.as_slice());

		let mut top_encoded_attributes = Vec::new();
		let _ = attributes.top_encode(&mut top_encoded_attributes);
		arg_buffer.push_argument_bytes(top_encoded_attributes.as_slice());

		// The API function has the last argument as variadic,
		// so we top-encode each and send as separate argument
		for uri in uris {
			let mut top_encoded_uri = Vec::new();
			let _ = uri.top_encode(&mut top_encoded_uri);

			arg_buffer.push_argument_bytes(top_encoded_uri.as_slice());
		}

		let output = self.call_local_esdt_built_in_function(
			self.get_gas_left(),
			b"ESDTNFTCreate",
			&arg_buffer,
		);

		u64::top_decode(output[0].as_slice()).unwrap_or_default()
	}
}
