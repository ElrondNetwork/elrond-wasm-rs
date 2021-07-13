use super::RustBigUint;
use crate::{TxContext, TxPanic};
use elrond_wasm::api::CallValueApi;
use elrond_wasm::err_msg;
use elrond_wasm::types::{EsdtTokenType, TokenIdentifier};

impl CallValueApi for TxContext {
	type AmountType = RustBigUint;

	fn check_not_payable(&self) {
		if self.egld_value() > 0 {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_EGLD.to_vec(),
			});
		}
		if self.esdt_value() > 0 {
			std::panic::panic_any(TxPanic {
				status: 10,
				message: err_msg::NON_PAYABLE_FUNC_ESDT.to_vec(),
			});
		}
	}

	#[inline]
	fn egld_value(&self) -> RustBigUint {
		self.tx_input_box.call_value.clone().into()
	}

	#[inline]
	fn esdt_value(&self) -> RustBigUint {
		self.tx_input_box.esdt_value.clone().into()
	}

	#[inline]
	fn token(&self) -> TokenIdentifier {
		TokenIdentifier::from(self.tx_input_box.esdt_token_identifier.as_slice())
	}

	#[inline]
	fn esdt_token_nonce(&self) -> u64 {
		// TODO: Add ESDT nonce in mock
		0u64
	}

	#[inline]
	fn esdt_token_type(&self) -> EsdtTokenType {
		// TODO: Add ESDT token type in mock
		EsdtTokenType::Fungible
	}
}
