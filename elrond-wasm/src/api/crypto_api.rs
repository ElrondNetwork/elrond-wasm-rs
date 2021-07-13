use super::BigUintApi;
use crate::types::H256;

pub trait CryptoApi {
	/// Numeric type used in some of the Arwen hooks.
	type BigUint: BigUintApi + 'static;

	fn sha256(&self, data: &[u8]) -> H256;

	fn keccak256(&self, data: &[u8]) -> H256;

	fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	/// Note: the signature is minimum 2 bytes in length,
	/// the second byte encodes the length of the remaining signature bytes.
	fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;
}
