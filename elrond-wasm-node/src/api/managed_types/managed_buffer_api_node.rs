use alloc::string::String;
use elrond_wasm::api::ManagedBufferApi;
use elrond_wasm::types::BoxedBytes;

// extern int32_t byteBufferNew(void* context, int32_t byteOffset, int32_t byteLength);
//
// extern int32_t byteBufferLength(void* context, int32_t reference);
// extern int32_t byteBufferGet(void* context, int32_t reference, int32_t byteOffset);
// extern void byteBufferSet(void* context, int32_t destination, int32_t byteOffset, int32_t byteLength);

extern "C" {
	fn byteBufferNew(byte_ptr: *const u8, byte_len: i32) -> i32;

	fn byteBufferLength(x: i32) -> i32;
	fn byteBufferGet(reference: i32, byte_ptr: *mut u8) -> i32;
	fn byteBufferSet(destination: i32, byte_ptr: *const u8, byte_len: i32);
}

pub struct ArwenManagedBuffer {
	pub handle: i32, // TODO: fix visibility
}

impl elrond_wasm::abi::TypeAbi for ArwenManagedBuffer {
	fn type_name() -> String {
		String::from("bytes")
	}
}

/// A raw bytes buffer managed by Arwen.
impl ManagedBufferApi for ArwenManagedBuffer {
	fn new(bytes: &[u8]) -> Self {
		unsafe {
			ArwenManagedBuffer {
				handle: byteBufferNew(bytes.as_ptr(), bytes.len() as i32),
			}
		}
	}

	fn len(&self) -> usize {
		unsafe { byteBufferLength(self.handle as i32) as usize }
	}

	fn overwrite(&mut self, bytes: &[u8]) {
		unsafe {
			byteBufferSet(self.handle as i32, bytes.as_ptr(), bytes.len() as i32);
		}
	}

	fn extend_from_slice(&mut self, slice: &[u8]) {
		panic!()
	}

	fn to_boxed_bytes(&self) -> BoxedBytes {
		panic!()
	}
}
