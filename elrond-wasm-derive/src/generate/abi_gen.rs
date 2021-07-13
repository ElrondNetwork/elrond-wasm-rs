use super::util::*;
use crate::model::{ContractTrait, Method, PublicRole};

fn generate_endpoint_snippet(m: &Method, endpoint_name: &str) -> proc_macro2::TokenStream {
	let endpoint_docs = &m.docs;
	let payable_in_tokens = m.payable_metadata().abi_strings();

	let input_snippets: Vec<proc_macro2::TokenStream> = m
		.method_args
		.iter()
		.filter_map(|arg| {
			if arg.metadata.payment.is_payment_arg() {
				None
			} else {
				let mut arg_type = arg.ty.clone();
				clear_all_type_lifetimes(&mut arg_type);
				let arg_name = &arg.pat;
				let arg_name_str = quote! { #arg_name }.to_string();
				Some(quote! {
					endpoint_abi.add_input::<#arg_type>(#arg_name_str);
					contract_abi.add_type_descriptions::<#arg_type>();
				})
			}
		})
		.collect();

	let output_names = &m.output_names;
	let output_snippet = match &m.return_type {
		syn::ReturnType::Default => quote! {},
		syn::ReturnType::Type(_, ty) => {
			let mut res_type = ty.clone();
			clear_all_type_lifetimes(&mut res_type);
			quote! {
				endpoint_abi.add_output::<#res_type>(&[ #(#output_names),* ]);
				contract_abi.add_type_descriptions::<#res_type>();
			}
		},
	};

	quote! {
		let mut endpoint_abi = elrond_wasm::abi::EndpointAbi{
			docs: &[ #(#endpoint_docs),* ],
			name: #endpoint_name,
			payable_in_tokens: &[ #(#payable_in_tokens),* ],
			inputs: Vec::new(),
			outputs: Vec::new(),
		};
		#(#input_snippets)*
		#output_snippet
	}
}

fn generate_abi_method_body(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let endpoint_snippets: Vec<proc_macro2::TokenStream> = contract
		.methods
		.iter()
		.filter_map(|m| {
			if let PublicRole::Init(_) = &m.public_role {
				let endpoint_def = generate_endpoint_snippet(m, "init");
				Some(quote! {
					#endpoint_def
					contract_abi.constructor = Some(endpoint_abi);
				})
			} else if let PublicRole::Endpoint(endpoint_metadata) = &m.public_role {
				let endpoint_name_str = endpoint_metadata.public_name.to_string();
				let endpoint_def = generate_endpoint_snippet(m, &endpoint_name_str);
				Some(quote! {
					#endpoint_def
					contract_abi.endpoints.push(endpoint_abi);
				})
			} else {
				None
			}
		})
		.collect();

	let supertrait_snippets: Vec<proc_macro2::TokenStream> = if is_contract_main {
		contract
			.supertraits
			.iter()
			.map(|supertrait| {
				let module_path = &supertrait.module_path;
				quote! {
					contract_abi.coalesce(<#module_path AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
				}
			})
			.collect()
	} else {
		Vec::new()
	};

	let contract_docs = &contract.docs;
	let contract_name = &contract.trait_name.to_string();
	quote! {
		let mut contract_abi = elrond_wasm::abi::ContractAbi{
			docs: &[ #(#contract_docs),* ],
			name: #contract_name,
			constructor: None,
			endpoints: Vec::new(),
			type_descriptions: <elrond_wasm::abi::TypeDescriptionContainerImpl as elrond_wasm::abi::TypeDescriptionContainer>::new(),
		};
		#(#endpoint_snippets)*
		#(#supertrait_snippets)*
		contract_abi
	}
}

pub fn generate_abi_provider(
	contract: &ContractTrait,
	is_contract_main: bool,
) -> proc_macro2::TokenStream {
	let abi_body = generate_abi_method_body(contract, is_contract_main);
	quote! {
		pub struct AbiProvider {}

		impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
			type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
			type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
			type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
			type Storage = elrond_wasm::api::uncallable::UncallableApi;
			type SendApi = elrond_wasm::api::uncallable::UncallableApi;

			fn abi() -> elrond_wasm::abi::ContractAbi {
				#abi_body
			}
		}
	}
}
