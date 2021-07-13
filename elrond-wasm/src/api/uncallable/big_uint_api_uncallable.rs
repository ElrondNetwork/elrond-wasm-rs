use crate::abi::TypeAbi;
use crate::api::BigUintApi;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

#[derive(Debug)]
pub struct BigUintUncallable;

impl TypeAbi for BigUintUncallable {
	fn type_name() -> String {
		String::from("BigUint")
	}
}

impl From<u64> for BigUintUncallable {
	fn from(_item: u64) -> Self {
		unreachable!()
	}
}

impl From<u32> for BigUintUncallable {
	fn from(_item: u32) -> Self {
		unreachable!()
	}
}

impl From<usize> for BigUintUncallable {
	fn from(_item: usize) -> Self {
		unreachable!()
	}
}

impl Clone for BigUintUncallable {
	fn clone(&self) -> Self {
		unreachable!()
	}
}

impl Default for BigUintUncallable {
	fn default() -> Self {
		unreachable!()
	}
}

macro_rules! binary_operator {
	($trait:ident, $method:ident) => {
		impl $trait for BigUintUncallable {
			type Output = BigUintUncallable;

			fn $method(self, _other: BigUintUncallable) -> BigUintUncallable {
				unreachable!()
			}
		}

		impl<'a, 'b> $trait<&'b BigUintUncallable> for &'a BigUintUncallable {
			type Output = BigUintUncallable;

			fn $method(self, _other: &BigUintUncallable) -> BigUintUncallable {
				unreachable!()
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

impl Sub for BigUintUncallable {
	type Output = BigUintUncallable;

	fn sub(self, _other: BigUintUncallable) -> BigUintUncallable {
		unreachable!()
	}
}

impl<'a, 'b> Sub<&'b BigUintUncallable> for &'a BigUintUncallable {
	type Output = BigUintUncallable;

	fn sub(self, _other: &BigUintUncallable) -> BigUintUncallable {
		unreachable!()
	}
}

macro_rules! binary_assign_operator {
	($trait:ident, $method:ident) => {
		impl $trait<BigUintUncallable> for BigUintUncallable {
			fn $method(&mut self, _other: Self) {
				unreachable!()
			}
		}

		impl $trait<&BigUintUncallable> for BigUintUncallable {
			fn $method(&mut self, _other: &BigUintUncallable) {
				unreachable!()
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

impl SubAssign<BigUintUncallable> for BigUintUncallable {
	fn sub_assign(&mut self, _other: Self) {
		unreachable!()
	}
}

impl SubAssign<&BigUintUncallable> for BigUintUncallable {
	fn sub_assign(&mut self, _other: &BigUintUncallable) {
		unreachable!()
	}
}

macro_rules! shift_traits {
	($shift_trait:ident, $method:ident) => {
		impl $shift_trait<usize> for BigUintUncallable {
			type Output = BigUintUncallable;

			fn $method(self, _rhs: usize) -> BigUintUncallable {
				unreachable!()
			}
		}

		impl<'a> $shift_trait<usize> for &'a BigUintUncallable {
			type Output = BigUintUncallable;

			fn $method(self, _rhs: usize) -> BigUintUncallable {
				unreachable!()
			}
		}
	};
}

shift_traits! {Shl, shl}
shift_traits! {Shr, shr}

macro_rules! shift_assign_traits {
	($shift_assign_trait:ident, $method:ident) => {
		impl $shift_assign_trait<usize> for BigUintUncallable {
			fn $method(&mut self, _rhs: usize) {
				unreachable!()
			}
		}
	};
}

shift_assign_traits! {ShlAssign, shl_assign}
shift_assign_traits! {ShrAssign, shr_assign}

impl PartialEq<Self> for BigUintUncallable {
	#[inline]
	fn eq(&self, _other: &Self) -> bool {
		unreachable!()
	}
}

impl Eq for BigUintUncallable {}

impl PartialOrd<Self> for BigUintUncallable {
	#[inline]
	fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
		unreachable!()
	}
}

impl Ord for BigUintUncallable {
	#[inline]
	fn cmp(&self, _other: &Self) -> Ordering {
		unreachable!()
	}
}

impl PartialEq<u64> for BigUintUncallable {
	#[inline]
	fn eq(&self, _other: &u64) -> bool {
		unreachable!()
	}
}

impl PartialOrd<u64> for BigUintUncallable {
	#[inline]
	fn partial_cmp(&self, _other: &u64) -> Option<Ordering> {
		unreachable!()
	}
}

use crate::elrond_codec::*;

impl NestedEncode for BigUintUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

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

impl TopEncode for BigUintUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

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

impl NestedDecode for BigUintUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

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

impl TopDecode for BigUintUncallable {
	const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

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

impl BigUintApi for BigUintUncallable {
	fn byte_length(&self) -> i32 {
		unreachable!()
	}

	fn copy_to_slice_big_endian(&self, _slice: &mut [u8]) -> i32 {
		unreachable!()
	}

	fn copy_to_array_big_endian_pad_right(&self, _target: &mut [u8; 32]) {
		unreachable!()
	}

	fn to_bytes_be(&self) -> Vec<u8> {
		unreachable!()
	}

	fn to_bytes_be_pad_right(&self, _nr_bytes: usize) -> Option<Vec<u8>> {
		unreachable!()
	}

	fn from_bytes_be(_bytes: &[u8]) -> Self {
		unreachable!()
	}

	fn sqrt(&self) -> Self {
		unreachable!()
	}

	fn pow(&self, _exp: u32) -> Self {
		unreachable!()
	}

	fn log2(&self) -> u32 {
        unreachable!()
	}
    
	fn to_u64(&self) -> Option<u64> {
		unreachable!()
	}
}
