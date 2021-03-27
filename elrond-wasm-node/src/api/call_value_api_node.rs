use super::managed_types::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::CallValueApi;
use elrond_wasm::types::{BoxedBytes, EsdtTokenType, TokenIdentifier};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
	fn checkNoPayment();

	fn bigIntNew(value: i64) -> i32;

	fn bigIntGetCallValue(dest: i32);
	fn bigIntGetESDTCallValue(dest: i32);
	fn getESDTTokenName(resultOffset: *const u8) -> i32;
	fn getESDTTokenNonce() -> u64;
	fn getESDTTokenType() -> i32;

	/// TODO: decide if it is worth using or not
	#[allow(dead_code)]
	fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi<ArwenBigUint> for ArwenApiImpl {
	#[inline]
	fn check_not_payable(&self) {
		unsafe {
			checkNoPayment();
		}
	}

	fn egld_value(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn esdt_value(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetESDTCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn token(&self) -> TokenIdentifier {
		unsafe {
			let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
			let name_len = getESDTTokenName(name_buffer.as_mut_ptr());
			if name_len == 0 {
				TokenIdentifier::egld()
			} else {
				BoxedBytes::from(&name_buffer[..name_len as usize]).into()
			}
		}
	}

	fn esdt_token_nonce(&self) -> u64 {
		unsafe { getESDTTokenNonce() }
	}

	fn esdt_token_type(&self) -> EsdtTokenType {
		unsafe {
			(getESDTTokenType() as u8).into()
		}
	}
}
