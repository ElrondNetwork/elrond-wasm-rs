use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/basic-features.wasm",
		Box::new(|context| Box::new(basic_features::contract_obj(context))),
	);
	contract_map
}

#[test]
fn big_int_to_i64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/big_int_to_i64.scen.json", &contract_map());
}

#[test]
fn big_uint_to_u64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/big_uint_to_u64.scen.json", &contract_map());
}

#[test]
fn block_info_rs() {
	elrond_wasm_debug::mandos_rs("mandos/block_info.scen.json", &contract_map());
}

#[test]
fn boxed_bytes_zeros_rs() {
	elrond_wasm_debug::mandos_rs("mandos/boxed_bytes_zeros.scen.json", &contract_map());
}

#[test]
fn count_ones_rs() {
	elrond_wasm_debug::mandos_rs("mandos/count_ones.scen.json", &contract_map());
}

#[test]
fn crypto_keccak256_rs() {
	elrond_wasm_debug::mandos_rs("mandos/crypto_keccak256.scen.json", &contract_map());
}

#[test]
fn crypto_sha256_rs() {
	elrond_wasm_debug::mandos_rs("mandos/crypto_sha256.scen.json", &contract_map());
}

#[test]
fn echo_array_u8_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_array_u8.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_async_result_empty.scen.json", &contract_map());
}

#[test]
fn echo_big_int_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_big_int.scen.json", &contract_map());
}

#[test]
fn echo_big_uint_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_big_uint.scen.json", &contract_map());
}

#[test]
fn echo_boxed_bytes_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_boxed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_i32_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_i32.scen.json", &contract_map());
}

#[test]
fn echo_i64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_i64.scen.json", &contract_map());
}

#[test]
fn echo_nothing_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_nothing.scen.json", &contract_map());
}

#[test]
fn echo_ser_ex_1_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_ser_ex_1.scen.json", &contract_map());
}

#[test]
fn echo_slice_u8_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_slice_u8.scen.json", &contract_map());
}

#[test]
fn echo_str_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_str.scen.json", &contract_map());
}

#[test]
fn echo_str_box_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_str_box.scen.json", &contract_map());
}

#[test]
fn echo_string_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_string.scen.json", &contract_map());
}

#[test]
fn echo_u64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_u64.scen.json", &contract_map());
}

#[test]
fn echo_usize_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_usize.scen.json", &contract_map());
}

#[test]
fn echo_varags_tuples_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_varags_tuples.scen.json", &contract_map());
}

#[test]
fn echo_varargs_u32_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_varargs_u32.scen.json", &contract_map());
}

#[test]
fn echo_vec_u8_rs() {
	elrond_wasm_debug::mandos_rs("mandos/echo_vec_u8.scen.json", &contract_map());
}

#[test]
fn events_rs() {
	elrond_wasm_debug::mandos_rs("mandos/events.scen.json", &contract_map());
}

#[test]
fn events_legacy_rs() {
	elrond_wasm_debug::mandos_rs("mandos/events_legacy.scen.json", &contract_map());
}

#[test]
fn get_caller_rs() {
	elrond_wasm_debug::mandos_rs("mandos/get_caller.scen.json", &contract_map());
}

#[test]
fn get_cumulated_validator_rewards_rs() {
	elrond_wasm_debug::mandos_rs(
		"mandos/get_cumulated_validator_rewards.scen.json",
		&contract_map(),
	);
}

// TODO: uncomment after implemented the full ESDT format in mandos-rs
// #[test]
// fn get_esdt_local_roles_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/get_esdt_local_roles.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn panic_rs() {
	elrond_wasm_debug::mandos_rs("mandos/panic.scen.json", &contract_map());
}

#[test]
fn return_codes_rs() {
	elrond_wasm_debug::mandos_rs("mandos/return_codes.scen.json", &contract_map());
}

// TODO: Fix by implementing is_smart_contract mock
/*
#[test]
fn sc_properties_rs() {
	elrond_wasm_debug::mandos_rs("mandos/sc_properties.scen.json", &contract_map());
}
*/

#[test]
fn sc_result_rs() {
	elrond_wasm_debug::mandos_rs("mandos/sc_result.scen.json", &contract_map());
}

#[test]
fn storage_addr_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_addr.scen.json", &contract_map());
}

#[test]
fn storage_big_int_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_big_int.scen.json", &contract_map());
}

#[test]
fn storage_big_uint_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_big_uint.scen.json", &contract_map());
}

#[test]
fn storage_bool_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_bool.scen.json", &contract_map());
}

#[test]
fn storage_clear_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_clear.scen.json", &contract_map());
}

#[test]
fn storage_i64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_i64.scen.json", &contract_map());
}

#[test]
fn storage_i64_bad_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_i64_bad.scen.json", &contract_map());
}

#[test]
fn storage_map1_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_map1.scen.json", &contract_map());
}

#[test]
fn storage_map2_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_map2.scen.json", &contract_map());
}

#[test]
fn storage_map3_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_map3.scen.json", &contract_map());
}

#[test]
fn storage_mapper_linked_list_rs() {
	elrond_wasm_debug::mandos_rs(
		"mandos/storage_mapper_linked_list.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_map_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_mapper_map.scen.json", &contract_map());
}

#[test]
fn storage_mapper_set_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_mapper_set.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_storage_rs() {
	elrond_wasm_debug::mandos_rs(
		"mandos/storage_mapper_map_storage.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_single_value_rs() {
	elrond_wasm_debug::mandos_rs(
		"mandos/storage_mapper_single_value.scen.json",
		&contract_map(),
	);
}

#[test]
fn storage_mapper_vec_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_mapper_vec.scen.json", &contract_map());
}

#[test]
fn storage_opt_addr_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_opt_addr.scen.json", &contract_map());
}

#[test]
fn storage_reserved_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_reserved.scen.json", &contract_map());
}

#[test]
fn storage_u64_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_u64.scen.json", &contract_map());
}

#[test]
fn storage_u64_bad_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_u64_bad.scen.json", &contract_map());
}

#[test]
fn storage_usize_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_usize.scen.json", &contract_map());
}

#[test]
fn storage_usize_bad_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_usize_bad.scen.json", &contract_map());
}

#[test]
fn storage_vec_u8_rs() {
	elrond_wasm_debug::mandos_rs("mandos/storage_vec_u8.scen.json", &contract_map());
}

#[test]
fn sqrt() {
	elrond_wasm_debug::mandos_rs("mandos/sqrt.scen.json", &contract_map());
}

#[test]
fn log2() {
	elrond_wasm_debug::mandos_rs("mandos/log2_func.scen.json", &contract_map());
}

#[test]
fn pow() {
	elrond_wasm_debug::mandos_rs("mandos/pow_func.scen.json", &contract_map());
}
