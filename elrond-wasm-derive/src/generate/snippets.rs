pub fn where_self_big_int() -> proc_macro2::TokenStream {
	quote! {
		where
			Self::BigUint: elrond_wasm::api::BigUintApi,
			for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
			for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
			for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
			for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
			for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
			for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
			Self::BigInt: elrond_wasm::api::BigIntApi,
			for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
			for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
			for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
			for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
	}
}

pub fn where_api_big_int() -> proc_macro2::TokenStream {
	quote! {
		where
			A::BigUint: elrond_wasm::api::BigUintApi,
			for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
			for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
			for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
			for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
			for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
			for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
			for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
			A::BigInt: elrond_wasm::api::BigIntApi,
			for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
			for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
			for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
			for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
	}
}

pub fn contract_object_def() -> proc_macro2::TokenStream {
	quote! {
		pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
			api: A,
		}
	}
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
	quote! {
		impl<A>elrond_wasm::api::ContractBase for ContractObj<A>
		where
			A:elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			type BigUint = A::BigUint;
			type BigInt = A::BigInt;
			type EllipticCurve = A::EllipticCurve;
			type Storage = A::Storage;
			type CallValue = A::CallValue;
			type SendApi = A::SendApi;
			type BlockchainApi = A::BlockchainApi;
			type CryptoApi = A::CryptoApi;
			type LogApi = A::LogApi;
			type ErrorApi = A::ErrorApi;

			#[inline]
			fn get_storage_raw(&self) -> Self::Storage {
				self.api.get_storage_raw()
			}
			#[inline]
			fn call_value(&self) -> Self::CallValue {
				self.api.call_value()
			}
			#[inline]
			fn send(&self) -> Self::SendApi {
				self.api.send()
			}
			#[inline]
			fn blockchain(&self) -> Self::BlockchainApi {
				self.api.blockchain()
			}
			#[inline]
			fn crypto(&self) -> Self::CryptoApi {
				self.api.crypto()
			}
			#[inline]
			fn log_api_raw(&self) -> Self::LogApi {
				self.api.log_api_raw()
			}
			#[inline]
			fn error_api(&self) -> Self::ErrorApi {
				self.api.error_api()
			}
		}
	}
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
	let where_api_big_int = where_api_big_int();
	quote! {
		pub fn contract_obj<A>(api: A) -> ContractObj<A>
		#where_api_big_int
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			ContractObj { api }
		}
	}
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
	quote! {
		impl<A> AutoImpl for ContractObj<A> where
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static
		{
		}
	}
}
pub fn impl_private_api() -> proc_macro2::TokenStream {
	quote! {
		impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
		where
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			type ArgumentApi = A;
			type FinishApi = A;

			#[inline]
			fn argument_api(&self) -> Self::ArgumentApi {
				self.api.clone()
			}

			#[inline]
			fn finish_api(&self) -> Self::FinishApi {
				self.api.clone()
			}
		}
	}
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
	let where_api_big_int = where_api_big_int();
	quote! {
		impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
		#where_api_big_int
			A: elrond_wasm::api::ContractBase
				+ elrond_wasm::api::ErrorApi
				+ elrond_wasm::api::EndpointArgumentApi
				+ elrond_wasm::api::EndpointFinishApi
				+ Clone
				+ 'static,
		{
			fn call(&self, fn_name: &[u8]) -> bool {
				EndpointWrappers::call(self, fn_name)
			}
			fn into_api(self: Box<Self>) -> A {
				self.api
			}
		}
	}
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
	quote! {
		pub struct Proxy<SA>
		where
			SA: elrond_wasm::api::SendApi + 'static,
		{
			pub api: SA,
			pub address: Address,
			pub payment_token: elrond_wasm::types::TokenIdentifier,
			pub payment_amount: SA::AmountType,
			pub payment_nonce: u64,
		}

		impl<SA> elrond_wasm::api::ProxyObjApi for Proxy<SA>
		where
			SA: elrond_wasm::api::SendApi + 'static,
		{
			type BigUint = SA::AmountType;
			type BigInt = SA::ProxyBigInt;
			type EllipticCurve = SA::ProxyEllipticCurve;
			type Storage = SA::ProxyStorage;
			type SendApi = SA;

			fn new_proxy_obj(api: SA, address: Address) -> Self {
				Proxy {
					api,
					address,
					payment_token: elrond_wasm::types::TokenIdentifier::egld(),
					payment_amount: Self::BigUint::zero(),
					payment_nonce: 0,
				}
			}

			fn with_token_transfer(mut self, token: TokenIdentifier, payment: Self::BigUint) -> Self {
				self.payment_token = token;
				self.payment_amount = payment;
				self
			}

			#[inline]
			fn with_nft_nonce(mut self, nonce: u64) -> Self {
				self.payment_nonce = nonce;
				self
			}

			#[inline]
			fn into_fields(self) -> (Self::SendApi, Address, TokenIdentifier, Self::BigUint, u64) {
				(
					self.api,
					self.address,
					self.payment_token,
					self.payment_amount,
					self.payment_nonce,
				)
			}
		}
	}
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
	quote! {
		pub struct CallbackProxyObj<SA>
		where
			SA: elrond_wasm::api::SendApi + 'static,
		{
			pub api: SA,
		}

		impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
		where
			SA: elrond_wasm::api::SendApi + 'static,
		{
			type BigUint = SA::AmountType;
			type BigInt = SA::ProxyBigInt;
			type EllipticCurve = SA::ProxyEllipticCurve;
			type Storage = SA::ProxyStorage;
			type SendApi = SA;
			type ErrorApi = SA;

			fn new_cb_proxy_obj(api: SA) -> Self {
				CallbackProxyObj {
					api,
				}
			}

			fn into_api(self) -> Self::ErrorApi {
				self.api
			}
		}
	}
}
