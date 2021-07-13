use super::BigUintUncallable;
use crate::abi::TypeAbi;
use crate::api::EllipticCurveApi;
use crate::elrond_codec::*;
use crate::types::BoxedBytes;
use alloc::string::String;

/// Dummy type that implements `EllipticCurveApi`.
/// Currently used to simplify generating ABIs, since we are not interested in values there.
/// Being completely content-less it can exist in `elrond-wasm` in a no-std environment.

type EllipticCurveComponents<BigUint> = (BigUint, BigUint, BigUint, BigUint, BigUint, u32);
pub struct EllipticCurveUncallable;

impl TypeAbi for EllipticCurveUncallable {
	fn type_name() -> String {
		String::from("EllipticCurve")
	}
}

impl NestedEncode for EllipticCurveUncallable {
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

impl TopEncode for EllipticCurveUncallable {
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

impl EllipticCurveApi for EllipticCurveUncallable {
	type BigUint = BigUintUncallable;

	fn get_values(&self) -> EllipticCurveComponents<Self::BigUint> {
		unreachable!()
	}

	fn p224_ec() -> Self {
		unreachable!()
	}

	fn p256_ec() -> Self {
		unreachable!()
	}

	fn p384_ec() -> Self {
		unreachable!()
	}

	fn p521_ec() -> Self {
		unreachable!()
	}

	fn get_ec_length(&self) -> u32 {
		unreachable!()
	}

    fn get_priv_key_byte_length(&self) -> u32 {
		unreachable!()
	}

	fn add_ec(
		&self,
		_x_first_point: Self::BigUint,
		_y_first_point: Self::BigUint,
		_x_second_point: Self::BigUint,
		_y_second_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn double_ec(
		&self,
		_x_point: Self::BigUint,
		_y_point: Self::BigUint,
	) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn is_on_curve_ec(&self, _x_point: Self::BigUint, _y_point: Self::BigUint) -> bool {
		unreachable!()
	}

	fn scalar_mult(
		&self,
		_x_point: Self::BigUint,
		_y_point: Self::BigUint,
		_data: BoxedBytes,
	) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn scalar_base_mult(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn marshal_ec(&self, _x_pair: Self::BigUint, _y_pair: Self::BigUint) -> BoxedBytes {
		unreachable!()
	}

	fn marshal_compressed_ec(&self, _x_pair: Self::BigUint, _y_pair: Self::BigUint) -> BoxedBytes {
		unreachable!()
	}

	fn unmarshal_ec(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn unmarshal_compressed_ec(&self, _data: BoxedBytes) -> (Self::BigUint, Self::BigUint) {
		unreachable!()
	}

	fn generate_key_ec(&self) -> (Self::BigUint, Self::BigUint, BoxedBytes) {
		unreachable!()
	}

    fn from_bitsize_ec(_bitsize: u32) -> Option<Self> {
        unreachable!()
    }
}
