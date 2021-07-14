use super::sc_error::SCError;
use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::EndpointFinishApi;
use crate::EndpointResult;
use crate::*;
use core::convert;
use core::ops::{ControlFlow, FromResidual, Try};

/// Default way to optionally return an error from a smart contract endpoint.
#[must_use]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SCResult<T> {
	Ok(T),
	Err(SCError),
}

impl<T> SCResult<T> {
	pub fn is_ok(&self) -> bool {
		matches!(self, SCResult::Ok(_))
	}

	pub fn is_err(&self) -> bool {
		!self.is_ok()
	}

	#[inline]
	pub fn ok(self) -> Option<T> {
		if let SCResult::Ok(t) = self {
			Some(t)
		} else {
			None
		}
	}

	#[inline]
	pub fn err(self) -> Option<SCError> {
		if let SCResult::Err(e) = self {
			Some(e)
		} else {
			None
		}
	}

	/// Used to convert from a regular Rust result.
	/// Any error type is accepted as long as it can be converted to a SCError
	/// (`Vec<u8>`, `&[u8]`, `BoxedBytes`, `String`, `&str` are covered).
	pub fn from_result<E>(r: core::result::Result<T, E>) -> Self
	where
		E: Into<SCError>,
	{
		match r {
			Ok(t) => SCResult::Ok(t),
			Err(e) => SCResult::Err(e.into()),
		}
	}
}

/// Implementing the `Try` trait overloads the `?` operator.
/// Documentation on the new version of the trait:
/// <https://github.com/scottmcm/rfcs/blob/do-or-do-not/text/0000-try-trait-v2.md#the-try-trait>
impl<T> Try for SCResult<T> {
	type Output = T;
	type Residual = SCError;

	fn branch(self) -> ControlFlow<Self::Residual, T> {
		match self {
			SCResult::Ok(t) => ControlFlow::Continue(t),
			SCResult::Err(e) => ControlFlow::Break(e),
		}
	}
	fn from_output(v: T) -> Self {
		SCResult::Ok(v)
	}
}

impl<T> FromResidual for SCResult<T> {
	fn from_residual(r: SCError) -> Self {
		SCResult::Err(r)
	}
}

impl<T, E> FromResidual<Result<convert::Infallible, E>> for SCResult<T>
where
	E: Into<SCError>,
{
	fn from_residual(residual: Result<convert::Infallible, E>) -> Self {
		match residual {
			Ok(_) => unreachable!(),
			Err(e) => SCResult::Err(e.into()),
		}
	}
}

impl<T> EndpointResult for SCResult<T>
where
	T: EndpointResult,
{
	/// Error implies the transaction fails, so if there is a result,
	/// it is of type `T`.
	type DecodeAs = T::DecodeAs;

	#[inline]
	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
		match self {
			SCResult::Ok(t) => {
				t.finish(api);
			},
			SCResult::Err(e) => {
				api.signal_error(e.as_bytes());
			},
		}
	}
}

impl<T: TypeAbi> TypeAbi for SCResult<T> {
	fn type_name() -> String {
		T::type_name()
	}

	/// Gives `SCResult<()>` the possibility to produce 0 output ABIs,
	/// just like `()`.
	/// It is also possible to have `SCResult<MultiResultX<...>>`,
	/// so this gives the MultiResult to dissolve into its multiple output ABIs.
	fn output_abis(output_names: &[&'static str]) -> Vec<OutputAbi> {
		T::output_abis(output_names)
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator);
	}
}

impl<T> SCResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			SCResult::Ok(t) => t,
			SCResult::Err(_) => panic!("called `SCResult::unwrap()`"),
		}
	}
}

impl<T> From<SCResult<T>> for Result<T, SCError> {
	fn from(result: SCResult<T>) -> Self {
		match result {
			SCResult::Ok(ok) => Result::Ok(ok),
			SCResult::Err(error) => Result::Err(error),
		}
	}
}

impl<T, Err> From<Result<T, Err>> for SCResult<T>
where
	Err: Into<SCError>,
{
	fn from(result: Result<T, Err>) -> Self {
		match result {
			Result::Ok(ok) => SCResult::Ok(ok),
			Result::Err(err) => SCResult::Err(err.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use elrond_codec::DecodeError;

use super::*;

	#[test]
	fn test_result_to_sc_result() {
		let result_ok: Result<i32, DecodeError> = Result::Ok(5);
		let sc_result_ok: SCResult<i32> = result_ok.into();

		assert!(sc_result_ok.unwrap() == 5);

		let result_err: Result<i32, DecodeError> = Result::Err(DecodeError::from(&b"Decode Error"[..]).into());
		let sc_result_err: SCResult<i32> = result_err.into();

		assert!(sc_result_err.err().unwrap().as_bytes() == &b"Decode Error"[..]);
	}
}
