#![no_std]
#![feature(new_uninit)]

pub mod api;
pub mod error_hook;
mod unsafe_buffer;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::vec::Vec;

/// The reference to the API implementation based on Arwen hooks.
/// It continas no data, can be embedded at no cost.
/// Cloning it is a no-op.
pub struct ArwenApiImpl {}

/// Should be no-op. The API implementation is zero-sized.
impl Clone for ArwenApiImpl {
	#[inline]
	fn clone(&self) -> Self {
		ArwenApiImpl {}
	}
}
