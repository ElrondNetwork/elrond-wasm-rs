use crate::types::BoxedBytes;
use alloc::string::String;
use alloc::vec::Vec;
use elrond_codec::{DecodeError, EncodeError};

/// Contains a smart contract execution error message.
///
/// Implemented as a simple boxed slice, for performance reasons.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SCError(BoxedBytes);

impl SCError {
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_slice()
	}
}

impl From<BoxedBytes> for SCError {
	#[inline]
	fn from(boxed_bytes: BoxedBytes) -> Self {
		SCError(boxed_bytes)
	}
}

impl From<&str> for SCError {
	#[inline]
	fn from(s: &str) -> Self {
		SCError(BoxedBytes::from(s.as_bytes()))
	}
}

impl From<String> for SCError {
	#[inline]
	fn from(s: String) -> Self {
		// data copy is avoided:
		// - String -> Vec<u8> via String::into_bytes is just a move
		// - Vec<u8> -> Box<[u8]> -> BoxedBytes is also just a move
		SCError(BoxedBytes::from(s.into_bytes()))
	}
}

impl From<&[u8]> for SCError {
	#[inline]
	fn from(byte_slice: &[u8]) -> Self {
		SCError(BoxedBytes::from(byte_slice))
	}
}

impl From<Vec<u8>> for SCError {
	#[inline]
	fn from(v: Vec<u8>) -> Self {
		SCError(BoxedBytes::from(v))
	}
}

impl From<EncodeError> for SCError {
	#[inline]
	fn from(err: EncodeError) -> Self {
		SCError::from(err.message_bytes())
	}
}

impl From<DecodeError> for SCError {
	#[inline]
	fn from(err: DecodeError) -> Self {
		SCError::from(err.message_bytes())
	}
}

impl From<!> for SCError {
	fn from(_: !) -> Self {
		unreachable!()
	}
}
