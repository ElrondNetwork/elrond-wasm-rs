use crate::abi;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

use alloc::vec::Vec;

/// Definition of the BigUint type required by the API.
/// The API doesn't care about the actual BigInt implementation.
/// The Arwen VM provides an implementation directly in the protocol.
/// For debugging we use a different implementation, based on Rust's BigInt.
///
/// Since most values in smart contracts will not be signed, as well as for safety,
/// most of the functionality if provided for unsigned integers.
pub trait BigUintApi:
	Sized
	+ From<u64>
	+ From<u32>
	+ From<usize>
	+ Clone
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ Rem<Output = Self>
	+ RemAssign
	+ BitAnd<Output = Self>
	+ BitAndAssign
	+ BitOr<Output = Self>
	+ BitOrAssign
	+ BitXor<Output = Self>
	+ BitXorAssign
	+ Shr<usize, Output = Self>
	+ ShrAssign<usize>
	+ Shl<usize, Output = Self>
	+ ShlAssign<usize>
	+ PartialEq<Self>
	+ Eq
	+ PartialOrd<Self>
	+ Ord
	+ PartialEq<u64>
	+ PartialOrd<u64>
	+ elrond_codec::NestedEncode
	+ elrond_codec::TopEncode
	+ elrond_codec::NestedDecode
	+ elrond_codec::TopDecode
	+ abi::TypeAbi
{
	fn zero() -> Self {
		0u64.into()
	}

	fn byte_length(&self) -> i32;

	fn copy_to_slice_big_endian(&self, slice: &mut [u8]) -> i32;

	fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]);

	fn to_bytes_be(&self) -> Vec<u8>;

	fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Option<Vec<u8>>;

	fn from_bytes_be(bytes: &[u8]) -> Self;
}
