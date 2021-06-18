elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub enum ProgrammedCallType {
	SyncCall,
	AsyncCall,
	TransferExecute,
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct ProgrammedCall<BigUint: BigUintApi> {
	call_type: ProgrammedCallType,
	to: Address,
	payment_token: TokenIdentifier,
	payment_nonce: u64,
	payment_amount: BigUint,
}

const MAX_SYNC_CALL_DEPTH: usize = 10;

#[elrond_wasm_derive::module]
pub trait ForwarderProgrammedCallModule {
	#[proxy]
	fn self_proxy(&self, to: Address) -> crate::Proxy<Self::SendApi>;

	#[view]
	#[storage_mapper("programmed_calls")]
	fn programmed_calls(&self) -> LinkedListMapper<Self::Storage, ProgrammedCall<Self::BigUint>>;

	#[endpoint]
	fn add_programmed_call(
		&self,
		call_type: ProgrammedCallType,
		to: Address,
		payment_token: TokenIdentifier,
		payment_nonce: u64,
		payment_amount: Self::BigUint,
	) {
		self.programmed_calls().push_back(ProgrammedCall {
			call_type,
			to,
			payment_token,
			payment_nonce,
			payment_amount,
		});
	}

	#[endpoint]
	#[payable("*")]
	fn forward_programmed_calls(
		&self,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment_nonce] token_nonce: u64,
		#[payment_amount] token_payment: Self::BigUint,
		sync_call_depth: usize,
	) -> OptionalResult<AsyncCall<Self::SendApi>> {
		self.forward_programmed_calls_event(&token_identifier, token_nonce, &token_payment);

		while let Some(call) = self.programmed_calls().pop_front() {
			match call.call_type {
				ProgrammedCallType::SyncCall => {
					if sync_call_depth >= MAX_SYNC_CALL_DEPTH - 1 {
						return OptionalResult::None;
					}
					let _ = self
						.self_proxy(call.to)
						.forward_programmed_calls(
							call.payment_token,
							call.payment_nonce,
							call.payment_amount,
							sync_call_depth + 1,
						)
						.execute_on_dest_context();
				},
				ProgrammedCallType::AsyncCall => {
					return OptionalResult::Some(
						self.self_proxy(call.to)
							.forward_programmed_calls(
								call.payment_token,
								call.payment_nonce,
								call.payment_amount,
								0,
							)
							.async_call(),
					)
				},
				ProgrammedCallType::TransferExecute => {
					let () = self
						.self_proxy(call.to)
						.forward_programmed_calls(
							call.payment_token,
							call.payment_nonce,
							call.payment_amount,
							0,
						)
						.transfer_execute();
				},
			}
		}

		OptionalResult::None
	}

	#[event("forward_programmed_calls")]
	fn forward_programmed_calls_event(
		&self,
		#[indexed] token_identifier: &TokenIdentifier,
		#[indexed] token_nonce: u64,
		#[indexed] token_payment: &Self::BigUint,
	);
}
