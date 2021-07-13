elrond_wasm::imports!();

use crate::types::*;

/// Storage tests: direct load.
#[elrond_wasm_derive::module]
pub trait StorageLoadFeatures {
	#[endpoint]
	#[storage_get("big_uint")]
	fn load_big_uint(&self) -> Self::BigUint;

	#[endpoint]
	#[storage_get("big_int")]
	fn load_big_int(&self) -> Self::BigInt;
    
	#[endpoint]
	#[storage_get("u64")]
	fn load_u64(&self) -> u64;

	#[endpoint]
	#[storage_get("usize")]
	fn load_usize(&self) -> usize;

	#[endpoint]
	#[storage_get("i64")]
	fn load_i64(&self) -> i64;

	#[endpoint]
	#[storage_get("bool")]
	fn load_bool(&self) -> bool;

	#[endpoint]
	#[storage_get("vec_u8")]
	fn load_vec_u8(&self) -> Vec<u8>;

	#[endpoint]
	#[storage_get("addr")]
	fn load_addr(&self) -> Address;

	#[storage_get("opt_addr")]
	fn _get_opt_addr(&self) -> Option<Address>;

	#[endpoint]
	fn load_opt_addr(&self) -> OptionalResult<Address> {
		self._get_opt_addr().into()
	}

	#[view]
	#[storage_is_empty("opt_addr")]
	fn is_empty_opt_addr(&self) -> bool;

	#[endpoint]
	#[storage_get("nr_to_clear")]
	fn get_nr_to_clear(&self) -> u32;

	#[endpoint]
	#[storage_clear("nr_to_clear")]
	fn clear_storage_value(&self);

	#[endpoint]
	#[storage_get("ser_1")]
	fn load_ser_1(&self) -> SerExample1;

	#[endpoint]
	#[storage_get("ser_2")]
	fn load_ser_2(&self) -> SerExample2;

	#[endpoint]
	#[storage_get("map1")]
	fn load_map1(&self, addr: Address) -> Self::BigUint;

	#[endpoint]
	#[storage_get("map2")]
	fn load_map2(&self, addr1: &Address, addr2: &Address) -> Self::BigUint;

	#[endpoint]
	#[storage_get("map3")]
	fn load_map3(&self, x: usize) -> bool;
}
