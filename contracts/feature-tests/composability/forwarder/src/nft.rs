elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::storage;

// used as mock attributes for NFTs
#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

#[allow(clippy::too_many_arguments)]
#[elrond_wasm_derive::module]
pub trait ForwarderNftModule: storage::ForwarderStorageModule {
	#[view]
	fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> Self::BigUint {
		self.blockchain().get_esdt_balance(
			&self.blockchain().get_sc_address(),
			token_identifier,
			nonce,
		)
	}

	#[payable("EGLD")]
	#[endpoint]
	fn nft_issue(
		&self,
		#[payment] issue_cost: Self::BigUint,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
	) -> AsyncCall<Self::SendApi> {
		let caller = self.blockchain().get_caller();

		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.issue_non_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				NonFungibleTokenProperties {
					can_freeze: true,
					can_wipe: true,
					can_pause: true,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: true,
				},
			)
			.async_call()
			.with_callback(self.callbacks().nft_issue_callback(&caller))
	}

	#[callback]
	fn nft_issue_callback(
		&self,
		caller: &Address,
		#[call_result] result: AsyncCallResult<TokenIdentifier>,
	) {
		match result {
			AsyncCallResult::Ok(token_identifier) => {
				self.last_issued_token().set(&token_identifier);
				self.last_error_message().clear();
			},
			AsyncCallResult::Err(message) => {
				// return issue cost to the caller
				let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
				if token_identifier.is_egld() && returned_tokens > 0 {
					self.send().direct_egld(caller, &returned_tokens, &[]);
				}

				self.last_error_message().set(&message.err_msg);
			},
		}
	}

	#[endpoint]
	#[allow(clippy::too_many_arguments)]
	fn nft_create(
		&self,
		token_identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		color: Color,
		uri: BoxedBytes,
	) -> u64 {
		let token_nonce = self.send().esdt_nft_create::<Color>(
			&token_identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&color,
			&[uri],
		);

		self.create_event(&token_identifier, token_nonce, &amount);

		token_nonce
	}

	#[endpoint]
	fn nft_add_quantity(
		&self,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
	) {
		self.send()
			.esdt_local_mint(&token_identifier, nonce, &amount);
	}

	#[endpoint]
	fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) {
		self.send()
			.esdt_local_burn(&token_identifier, nonce, &amount);
	}

	#[endpoint]
	fn transfer_nft_via_async_call(
		&self,
		to: Address,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
		data: BoxedBytes,
	) {
		self.send().transfer_esdt_via_async_call(
			&to,
			&token_identifier,
			nonce,
			&amount,
			data.as_slice(),
		);
	}

	#[endpoint]
	fn transfer_nft_and_execute(
		&self,
		to: Address,
		token_identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
		function: BoxedBytes,
		#[var_args] arguments: VarArgs<BoxedBytes>,
	) {
		let mut arg_buffer = ArgBuffer::new();
		for arg in arguments.into_vec() {
			arg_buffer.push_argument_bytes(arg.as_slice());
		}

		let _ = self.send().direct_esdt_nft_execute(
			&to,
			&token_identifier,
			nonce,
			&amount,
			self.blockchain().get_gas_left(),
			function.as_slice(),
			&arg_buffer,
		);
	}

	#[endpoint]
	fn create_and_send(
		&self,
		to: Address,
		token_identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		color: Color,
		uri: BoxedBytes,
	) {
		let token_nonce = self.nft_create(
			token_identifier.clone(),
			amount.clone(),
			name,
			royalties,
			hash,
			color,
			uri,
		);

		self.send().direct(
			&to,
			&token_identifier,
			token_nonce,
			&amount,
			b"NFT transfer",
		);

		self.send_event(&to, &token_identifier, token_nonce, &amount);
	}

	#[event("create")]
	fn create_event(
		&self,
		#[indexed] token_id: &TokenIdentifier,
		#[indexed] token_nonce: u64,
		#[indexed] amount: &Self::BigUint,
	);

	#[event("send")]
	fn send_event(
		&self,
		#[indexed] to: &Address,
		#[indexed] token_id: &TokenIdentifier,
		#[indexed] token_nonce: u64,
		#[indexed] amount: &Self::BigUint,
	);
}
