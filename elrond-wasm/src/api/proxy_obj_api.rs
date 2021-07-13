use super::{
	BigIntApi, BigUintApi, EllipticCurveApi, ErrorApi, SendApi, StorageReadApi, StorageWriteApi,
};
use crate::types::{Address, TokenIdentifier};

pub trait ProxyObjApi {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	type EllipticCurve: EllipticCurveApi<BigUint = Self::BigUint> + 'static;

	/// The code generator produces the same types in the proxy, as for the main contract.
	/// Sometimes endpoints return types that contain a `Self::Storage` type argument,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	/// In order for the proxy code to compile, it is necessary to specify this type here too
	/// (even though it is not required by the trait's methods per se).
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	type SendApi: SendApi<AmountType = Self::BigUint, ProxyBigInt = Self::BigInt> + Clone + 'static;

	// type ContractCall<R>;

	fn new_proxy_obj(api: Self::SendApi, address: Address) -> Self;

	fn with_token_transfer(self, token: TokenIdentifier, payment: Self::BigUint) -> Self;

	fn with_nft_nonce(self, nonce: u64) -> Self;

	fn into_fields(self) -> (Self::SendApi, Address, TokenIdentifier, Self::BigUint, u64);
}

pub trait CallbackProxyObjApi {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	type EllipticCurve: EllipticCurveApi<BigUint = Self::BigUint> + 'static;

	/// The code generator produces the same types in the proxy, as for the main contract.
	/// Sometimes endpoints return types that contain a `Self::Storage` type argument,
	/// as for example in `SingleValueMapper<Self::Storage, i32>`.
	/// In order for the proxy code to compile, it is necessary to specify this type here too
	/// (even though it is not required by the trait's methods per se).
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	type SendApi: SendApi<AmountType = Self::BigUint, ProxyBigInt = Self::BigInt> + Clone + 'static;

	type ErrorApi: ErrorApi + Clone + 'static;

	fn new_cb_proxy_obj(api: Self::ErrorApi) -> Self;

	fn into_api(self) -> Self::ErrorApi;
}
