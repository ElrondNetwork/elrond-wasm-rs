use crate::api::RustBigUint;
use crate::TxContext;
use elrond_wasm::api::CryptoApi;
use elrond_wasm::types::H256;
use sha2::Sha256;
use sha3::{Digest, Keccak256};

impl CryptoApi for TxContext {
	type BigUint = RustBigUint;

	fn sha256(&self, data: &[u8]) -> H256 {
		let mut hasher = Sha256::new();
		hasher.update(data);
		let hash: [u8; 32] = hasher.finalize().into();
		hash.into()
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		let mut hasher = Keccak256::new();
		hasher.update(data);
		let hash: [u8; 32] = hasher.finalize().into();
		hash.into()
	}

	fn verify_bls(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_bls not implemented yet!")
	}

	fn verify_ed25519(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_ed25519 not implemented yet!")
	}

	fn verify_secp256k1(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
		panic!("verify_secp256k1 not implemented yet!")
	}
}
