use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use elrond_wasm::api::BigUintApi;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::pow;
use std::fmt;

#[derive(Debug)]
pub struct RustBigUint(pub num_bigint::BigInt);

impl RustBigUint {
	/// Convert to Rust BigUint.
	pub fn value(&self) -> BigUint {
		self.0.to_biguint().unwrap()
	}
}

impl fmt::Display for RustBigUint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.value().fmt(f)
	}
}

impl From<u64> for RustBigUint {
	fn from(item: u64) -> Self {
		RustBigUint(BigUint::from(item).into())
	}
}

impl From<u32> for RustBigUint {
	fn from(item: u32) -> Self {
		RustBigUint(BigUint::from(item).into())
	}
}

impl From<usize> for RustBigUint {
	fn from(item: usize) -> Self {
		RustBigUint(BigUint::from(item).into())
	}
}

impl From<BigInt> for RustBigUint {
	fn from(item: BigInt) -> Self {
		RustBigUint(item)
	}
}

impl From<BigUint> for RustBigUint {
	fn from(item: BigUint) -> Self {
		RustBigUint(BigInt::from_biguint(Sign::Plus, item))
	}
}

impl Clone for RustBigUint {
	fn clone(&self) -> Self {
		RustBigUint(self.0.clone())
	}
}

impl Default for RustBigUint {
	fn default() -> Self {
		Self::zero()
	}
}

macro_rules! binary_operator {
	($trait:ident, $method:ident) => {
		impl $trait for RustBigUint {
			type Output = RustBigUint;

			fn $method(self, other: RustBigUint) -> RustBigUint {
				RustBigUint((self.0).$method(other.0))
			}
		}

		impl<'a, 'b> $trait<&'b RustBigUint> for &'a RustBigUint {
			type Output = RustBigUint;

			fn $method(self, other: &RustBigUint) -> RustBigUint {
				RustBigUint(self.0.clone().$method(other.0.clone()))
			}
		}
	};
}

binary_operator! {Add, add}
binary_operator! {Mul, mul}
binary_operator! {Div, div}
binary_operator! {Rem, rem}

binary_operator! {BitAnd, bitand}
binary_operator! {BitOr, bitor}
binary_operator! {BitXor, bitxor}

fn check_sub_result(result: &BigInt) {
	if result.sign() == num_bigint::Sign::Minus {
		panic!("Cannot subtract because result would be negative")
	}
}

impl Sub for RustBigUint {
	type Output = RustBigUint;

	fn sub(self, other: RustBigUint) -> RustBigUint {
		let result = self.0 - other.0;
		check_sub_result(&result);
		RustBigUint(result)
	}
}

impl<'a, 'b> Sub<&'b RustBigUint> for &'a RustBigUint {
	type Output = RustBigUint;

	fn sub(self, other: &RustBigUint) -> RustBigUint {
		let result = self.0.clone().sub(other.0.clone());
		check_sub_result(&result);
		RustBigUint(result)
	}
}

macro_rules! binary_assign_operator {
	($trait:ident, $method:ident) => {
		impl $trait<RustBigUint> for RustBigUint {
			fn $method(&mut self, other: Self) {
				BigInt::$method(&mut self.0, other.0);
			}
		}

		impl $trait<&RustBigUint> for RustBigUint {
			fn $method(&mut self, other: &RustBigUint) {
				BigInt::$method(&mut self.0, &other.0);
			}
		}
	};
}

binary_assign_operator! {AddAssign, add_assign}
binary_assign_operator! {MulAssign, mul_assign}
binary_assign_operator! {DivAssign, div_assign}
binary_assign_operator! {RemAssign, rem_assign}

binary_assign_operator! {BitAndAssign, bitand_assign}
binary_assign_operator! {BitOrAssign,  bitor_assign}
binary_assign_operator! {BitXorAssign, bitxor_assign}

impl SubAssign<RustBigUint> for RustBigUint {
	fn sub_assign(&mut self, other: Self) {
		BigInt::sub_assign(&mut self.0, other.0);
		check_sub_result(&self.0);
	}
}

impl SubAssign<&RustBigUint> for RustBigUint {
	fn sub_assign(&mut self, other: &RustBigUint) {
		BigInt::sub_assign(&mut self.0, &other.0);
		check_sub_result(&self.0);
	}
}

macro_rules! shift_traits {
	($shift_trait:ident, $method:ident) => {
		impl $shift_trait<usize> for RustBigUint {
			type Output = RustBigUint;

			fn $method(self, rhs: usize) -> RustBigUint {
				let result = $shift_trait::$method(self.0, rhs);
				RustBigUint(result)
			}
		}

		impl<'a> $shift_trait<usize> for &'a RustBigUint {
			type Output = RustBigUint;

			fn $method(self, rhs: usize) -> RustBigUint {
				let result = $shift_trait::$method(&self.0, rhs);
				RustBigUint(result)
			}
		}
	};
}

shift_traits! {Shl, shl}
shift_traits! {Shr, shr}

macro_rules! shift_assign_traits {
	($shift_assign_trait:ident, $method:ident) => {
		impl $shift_assign_trait<usize> for RustBigUint {
			fn $method(&mut self, rhs: usize) {
				$shift_assign_trait::$method(&mut self.0, rhs);
			}
		}
	};
}

shift_assign_traits! {ShlAssign, shl_assign}
shift_assign_traits! {ShrAssign, shr_assign}

impl PartialEq<Self> for RustBigUint {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.0, &other.0)
	}
}

impl Eq for RustBigUint {}

impl PartialOrd<Self> for RustBigUint {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.0, &other.0)
	}
}

impl Ord for RustBigUint {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(&self.0, &other.0)
	}
}

impl PartialEq<u64> for RustBigUint {
	#[inline]
	fn eq(&self, other: &u64) -> bool {
		PartialEq::eq(&self.0, &BigInt::from(*other as i64))
	}
}

impl PartialOrd<u64> for RustBigUint {
	#[inline]
	fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.0, &BigInt::from(*other as i64))
	}
}

use elrond_wasm::elrond_codec::*;

impl NestedEncode for RustBigUint {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.to_bytes_be().as_slice().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_bytes_be()
			.as_slice()
			.dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for RustBigUint {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.to_bytes_be().top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_bytes_be().top_encode_or_exit(output, c, exit)
	}
}

impl NestedDecode for RustBigUint {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let size = usize::dep_decode(input)?;
		let bytes = input.read_slice(size)?;
		Ok(RustBigUint::from_bytes_be(bytes))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		let bytes = input.read_slice_or_exit(size, c, exit);
		RustBigUint::from_bytes_be(bytes)
	}
}

impl TopDecode for RustBigUint {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(RustBigUint::from_bytes_be(&*input.into_boxed_slice_u8()))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		RustBigUint::from_bytes_be(&*input.into_boxed_slice_u8())
	}
}

impl elrond_wasm::abi::TypeAbi for RustBigUint {
	fn type_name() -> String {
		String::from("BigUint")
	}
}

impl elrond_wasm::api::BigUintApi for RustBigUint {
	fn byte_length(&self) -> i32 {
		panic!("byte_length not yet implemented")
	}

	fn copy_to_slice_big_endian(&self, _slice: &mut [u8]) -> i32 {
		panic!("copy_to_slice not yet implemented")
	}

	fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
		if self.0.sign() == Sign::Plus {
			let (_, bytes) = self.0.to_bytes_be();
			let offset = 32 - bytes.len();
			target[offset..].clone_from_slice(&bytes[..]);
		}
	}

	fn to_bytes_be(&self) -> Vec<u8> {
		if self.0.sign() == Sign::NoSign {
			Vec::new()
		} else {
			let (_, be) = self.0.to_bytes_be();
			be
		}
	}

	fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Option<Vec<u8>> {
		let (_, bytes_be) = self.0.to_bytes_be();
		match bytes_be.len().cmp(&nr_bytes) {
			Ordering::Greater => None,
			Ordering::Equal => Some(bytes_be),
			Ordering::Less => {
				let mut res = vec![0u8; nr_bytes];
				let offset = nr_bytes - bytes_be.len();
				res[offset..].clone_from_slice(&bytes_be[..]);
				Some(res)
			},
		}
	}

	fn from_bytes_be(bytes: &[u8]) -> Self {
		let bi = BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes);
		bi.into()
	}

	fn sqrt(&self) -> Self {
		RustBigUint(self.0.sqrt())
	}

	fn pow(&self, exp: u32) -> Self {
		RustBigUint(pow(self.0.clone(), exp as usize))
	}

	fn log2(&self) -> u32 {
		(self.0.bits() as u32) - 1
    }
    
	fn to_u64(&self) -> Option<u64> {
		let (_, digits) = self.0.to_u64_digits();
		match digits.len() {
			0 => Some(0),
			1 => {
				let as_u64 = digits[0];
				if as_u64 <= i64::MAX as u64 {
					Some(digits[0])
				} else {
					None
				}
			},
			_ => None,
		}
	}
}
