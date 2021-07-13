use super::ArwenBigUint;
use crate::ArwenApiImpl;
use elrond_wasm::api::CryptoApi;
use elrond_wasm::types::H256;

extern "C" {

	fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

	fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

	fn verifyBLS(
		keyOffset: *const u8,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;

	fn verifyEd25519(
		keyOffset: *const u8,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;

	fn verifySecp256k1(
		keyOffset: *const u8,
		keyLength: i32,
		messageOffset: *const u8,
		messageLength: i32,
		sigOffset: *const u8,
	) -> i32;

}

impl CryptoApi for ArwenApiImpl {
	type BigUint = ArwenBigUint;

	fn sha256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		unsafe {
			let mut res = H256::zero();
			keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
			res
		}
	}

	// the verify functions return 0 if valid signature, -1 if invalid

	fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifyBLS(
				key.as_ptr(),
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}

	fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifyEd25519(
				key.as_ptr(),
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}

	fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
		unsafe {
			verifySecp256k1(
				key.as_ptr(),
				key.len() as i32,
				message.as_ptr(),
				message.len() as i32,
				signature.as_ptr(),
			) == 0
		}
	}
}
