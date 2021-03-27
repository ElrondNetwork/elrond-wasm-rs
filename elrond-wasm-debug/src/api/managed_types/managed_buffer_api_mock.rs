use elrond_wasm::types::BoxedBytes;

#[derive(Debug)]
pub struct RustManagedBuffer(pub Vec<u8>);

impl elrond_wasm::abi::TypeAbi for RustManagedBuffer {
	fn type_name() -> String {
		String::from("bytes")
	}
}

impl elrond_wasm::api::ManagedBufferApi for RustManagedBuffer {
	fn new(value: &[u8]) -> Self {
		RustManagedBuffer(value.to_vec())
	}

	fn len(&self) -> usize {
		self.0.len()
	}

	fn overwrite(&mut self, value: &[u8]) {
		self.0 = value.to_vec();
	}

	fn extend_from_slice(&mut self, other: &[u8]) {
		self.0.extend_from_slice(other)
	}

	fn to_boxed_bytes(&self) -> BoxedBytes {
		self.0.as_slice().into()
	}
}
