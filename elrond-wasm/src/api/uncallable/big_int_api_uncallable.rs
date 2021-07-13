use crate::abi::TypeAbi;
use crate::api::{BigIntApi, Sign};
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

/// Dummy type that implements `BigIntApi`.
/// Currently used to simplify generating ABIs, since we are not interested in values there.
/// Being completely content-less it can exist in `elrond-wasm` in a no-std environment.
pub struct BigIntUncallable;

impl TypeAbi for BigIntUncallable {
	fn type_name() -> String {
		String::from("BigInt")
	}
}

impl From<BigUintUncallable> for BigIntUncallable {
	fn from(_item: BigUintUncallable) -> Self {
		unreachable!()
	}
}

impl From<i64> for BigIntUncallable {
	fn from(_item: i64) -> Self {
		unreachable!()
	}
}

impl From<i32> for BigIntUncallable {
	fn from(_item: i32) -> Self {
		unreachable!()
	}
}

impl Clone for BigIntUncallable {
	fn clone(&self) -> Self {
		unreachable!()
	}
}

macro_rules! binary_operator {
	($trait:ident, $method:ident) => {
		impl $trait for BigIntUncallable {
			type Output = BigIntUncallable;

			fn $method(self, _other: BigIntUncallable) -> BigIntUncallable {
				unreachable!()
			}
		}

		impl<'a, 'b> $trait<&'b BigIntUncallable> for &'a BigIntUncallable {
			type Output = BigIntUncallable;

			fn $method(self, _other: &BigIntUncallable) -> BigIntUncallable {
				unreachable!()
			}
		}
	};
}

binary_operator! {Add, add}
binary_operator! {Sub, sub}
binary_operator! {Mul, mul}
binary_operator! {Div, div}
binary_operator! {Rem, rem}

macro_rules! binary_assign_operator {
	($trait:ident, $method:ident) => {
		impl $trait<BigIntUncallable> for BigIntUncallable {
			fn $method(&mut self, _other: Self) {
				unreachable!()
			}
		}

		impl $trait<&BigIntUncallable> for BigIntUncallable {
			fn $method(&mut self, _other: &BigIntUncallable) {
				unreachable!()
			}
		}
	};
}

binary_assign_operator! {AddAssign, add_assign}
binary_assign_operator! {SubAssign, sub_assign}
binary_assign_operator! {MulAssign, mul_assign}
binary_assign_operator! {DivAssign, div_assign}
binary_assign_operator! {RemAssign, rem_assign}

impl Neg for BigIntUncallable {
	type Output = BigIntUncallable;

	fn neg(self) -> Self::Output {
		unreachable!()
	}
}

impl PartialEq<Self> for BigIntUncallable {
	#[inline]
	fn eq(&self, _other: &Self) -> bool {
		unreachable!()
	}
}

impl Eq for BigIntUncallable {}

impl PartialOrd<Self> for BigIntUncallable {
	#[inline]
	fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
		unreachable!()
	}
}

impl Ord for BigIntUncallable {
	#[inline]
	fn cmp(&self, _other: &Self) -> Ordering {
		unreachable!()
	}
}

impl PartialEq<i64> for BigIntUncallable {
	#[inline]
	fn eq(&self, _other: &i64) -> bool {
		unreachable!()
	}
}

impl PartialOrd<i64> for BigIntUncallable {
	#[inline]
	fn partial_cmp(&self, _other: &i64) -> Option<Ordering> {
		unreachable!()
	}
}

use crate::elrond_codec::*;

use super::BigUintUncallable;

impl NestedEncode for BigIntUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_encode<O: NestedEncodeOutput>(&self, _dest: &mut O) -> Result<(), EncodeError> {
		unreachable!()
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		_dest: &mut O,
		_c: ExitCtx,
		_exit: fn(ExitCtx, EncodeError) -> !,
	) {
		unreachable!()
	}
}

impl TopEncode for BigIntUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn top_encode<O: TopEncodeOutput>(&self, _output: O) -> Result<(), EncodeError> {
		unreachable!()
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		_output: O,
		_c: ExitCtx,
		_exit: fn(ExitCtx, EncodeError) -> !,
	) {
		unreachable!()
	}
}

impl NestedDecode for BigIntUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_decode<I: NestedDecodeInput>(_input: &mut I) -> Result<Self, DecodeError> {
		unreachable!()
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		_input: &mut I,
		_c: ExitCtx,
		_exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		unreachable!()
	}
}

impl TopDecode for BigIntUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn top_decode<I: TopDecodeInput>(_input: I) -> Result<Self, DecodeError> {
		unreachable!()
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		_input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		unreachable!()
	}
}

impl BigIntApi for BigIntUncallable {
	type BigUint = BigUintUncallable;

	fn abs_uint(&self) -> Self::BigUint {
		unreachable!()
	}

	fn sign(&self) -> Sign {
		unreachable!()
	}

	fn to_signed_bytes_be(&self) -> Vec<u8> {
		unreachable!()
	}

	fn from_signed_bytes_be(_bytes: &[u8]) -> Self {
		unreachable!()
	}

	fn pow(&self, _exp: u32) -> Self {
        unreachable!()
	}
    
	fn to_i64(&self) -> Option<i64> {
		unreachable!()
	}
}
