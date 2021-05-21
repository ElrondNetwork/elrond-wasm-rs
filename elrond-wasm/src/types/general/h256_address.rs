use super::h256::H256;
use crate::abi::TypeAbi;
use crate::types::BoxedBytes;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Debug;

const LENGTH_IN_BYTES: usize = 32;

/// An Address is just a H256 with a different name.
/// Has a different ABI name than H256.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Address(H256);

impl From<H256> for Address {
	#[inline]
	fn from(hash: H256) -> Self {
		Address(hash)
	}
}

impl From<Address> for H256 {
	#[inline]
	fn from(address: Address) -> Self {
		address.0
	}
}

impl<'a> From<&'a Address> for &'a H256 {
	#[inline]
	fn from(address: &'a Address) -> Self {
		&address.0
	}
}

impl From<[u8; LENGTH_IN_BYTES]> for Address {
	#[inline]
	fn from(arr: [u8; LENGTH_IN_BYTES]) -> Self {
		Address(H256::from(arr))
	}
}

impl<'a> From<&'a [u8; LENGTH_IN_BYTES]> for Address {
	#[inline]
	fn from(bytes: &'a [u8; LENGTH_IN_BYTES]) -> Self {
		Address(H256::from(bytes))
	}
}

impl<'a> From<&'a mut [u8; LENGTH_IN_BYTES]> for Address {
	#[inline]
	fn from(bytes: &'a mut [u8; LENGTH_IN_BYTES]) -> Self {
		Address(H256::from(bytes))
	}
}

impl From<Box<[u8; LENGTH_IN_BYTES]>> for Address {
	#[inline]
	fn from(bytes: Box<[u8; LENGTH_IN_BYTES]>) -> Self {
		Address(H256::from(bytes))
	}
}

impl Address {
	pub fn from_slice(slice: &[u8]) -> Self {
		Address(H256::from_slice(slice))
	}
}

impl From<Address> for [u8; LENGTH_IN_BYTES] {
	#[inline]
	fn from(addr: Address) -> Self {
		addr.0.into()
	}
}

impl AsRef<[u8]> for Address {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

impl AsMut<[u8]> for Address {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		self.0.as_mut()
	}
}

impl Address {
	/// Returns a new address of 32 zeros.
	/// Allocates directly in heap.
	/// Minimal resulting wasm code (14 bytes if not inlined).
	pub fn zero() -> Self {
		Address(H256::zero())
	}

	/// Returns the size of an address in bytes.
	#[inline]
	pub fn len_bytes() -> usize {
		H256::len_bytes()
	}

	/// Extracts a byte slice containing the entire fixed hash.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}

	#[inline]
	pub fn copy_to_array(&self, target: &mut [u8; LENGTH_IN_BYTES]) {
		self.0.copy_to_array(target)
	}

	#[inline]
	pub fn to_vec(&self) -> Vec<u8> {
		self.0.to_vec()
	}

	/// Pointer to the data on the heap.
	#[inline]
	pub fn as_ptr(&mut self) -> *const u8 {
		self.0.as_ptr()
	}

	/// Returns an unsafe mutable pointer to the data on the heap.
	/// Used by the API to populate data.
	#[inline]
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.0.as_mut_ptr()
	}

	/// True if all 32 bytes of the hash are zero.
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	/// Transmutes self to an (in principle) variable length boxed bytes object.
	/// Both BoxedBytes and H256 keep the data on the heap, so only the pointer to that data needs to be transmuted.
	/// Does not reallocate or copy data, the data on the heap remains untouched.
	pub fn into_boxed_bytes(self) -> BoxedBytes {
		self.0.into_boxed_bytes()
	}
}

use elrond_codec::*;

impl NestedEncode for Address {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.0.dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.0.dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for Address {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.0.top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.0.top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for Address {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Address(H256::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Address(H256::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for Address {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(Address(H256::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Address(H256::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for Address {
	fn type_name() -> String {
		"Address".into()
	}
}

#[cfg(test)]
mod address_tests {
	use super::*;
	use alloc::vec::Vec;
	use elrond_codec::test_util::{check_top_encode, check_top_decode, ser_deser_ok};

	#[test]
	fn test_address() {
		let addr = Address::from([4u8; LENGTH_IN_BYTES]);
		ser_deser_ok(addr, &[4u8; LENGTH_IN_BYTES]);
	}

	#[test]
	fn test_opt_address() {
		let addr = Address::from([4u8; LENGTH_IN_BYTES]);
		let mut expected: Vec<u8> = Vec::new();
		expected.push(1u8);
		expected.extend_from_slice(&[4u8; LENGTH_IN_BYTES]);
		ser_deser_ok(Some(addr), expected.as_slice());
	}

	#[test]
	fn test_ser_address_ref() {
		let addr = Address::from([4u8; LENGTH_IN_BYTES]);
		let expected_bytes: &[u8] = &[4u8; LENGTH_IN_BYTES * 3];

		let tuple = (&addr, &&&addr, addr.clone());
		let serialized_bytes = check_top_encode(&tuple);
		assert_eq!(serialized_bytes.as_slice(), expected_bytes);
	}

	#[test]
	fn test_is_zero() {
		assert!(Address::zero().is_zero());
	}

	#[test]
	fn test_size_of() {
		use core::mem::size_of;
		assert_eq!(size_of::<Address>(), size_of::<usize>());
		assert_eq!(size_of::<Option<Address>>(), size_of::<usize>());
	}

	#[test]
	fn test_decode_from_empty() {
		let decoded = check_top_decode::<Address>(&[]);
		assert!(decoded.is_zero());
	}
}
