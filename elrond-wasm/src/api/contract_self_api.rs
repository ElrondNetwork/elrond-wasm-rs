use super::{
	BigIntApi, BigUintApi, BlockchainApi, CallValueApi, CryptoApi, EllipticCurveApi,
	EndpointArgumentApi, EndpointFinishApi, ErrorApi, LogApi, ProxyObjApi, SendApi, StorageReadApi,
	StorageWriteApi,
};
use crate::types::Address;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait ContractBase: Sized {
	type BigUint: BigUintApi + 'static;

	type BigInt: BigIntApi + 'static;

	type EllipticCurve: EllipticCurveApi<BigUint = Self::BigUint> + 'static;

	/// Abstracts the lower-level storage functionality.
	type Storage: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static;

	/// Abstracts the call value handling at the beginning of a function call.
	type CallValue: CallValueApi<AmountType = Self::BigUint> + ErrorApi + Clone + 'static;

	/// Abstracts the sending of EGLD & ESDT transactions, as well as async calls.
	type SendApi: SendApi<
			AmountType = Self::BigUint,
			ProxyBigInt = Self::BigInt,
			ProxyEllipticCurve = Self::EllipticCurve,
			ProxyStorage = Self::Storage,
		> + Clone
		+ 'static;

	type BlockchainApi: BlockchainApi<BalanceType = Self::BigUint> + Clone + 'static;

	type CryptoApi: CryptoApi<BigUint = Self::BigUint> + Clone + 'static;

	type LogApi: LogApi + ErrorApi + Clone + 'static;

	type ErrorApi: ErrorApi + Clone + 'static;

	/// Gateway into the lower-level storage functionality.
	/// Storage related annotations make use of this.
	/// Using it directly is not recommended.
	fn get_storage_raw(&self) -> Self::Storage;

	/// Gateway into the call value retrieval functionality.
	/// The payment annotations should normally be the ones to handle this,
	/// but the developer is also given direct access to the API.
	fn call_value(&self) -> Self::CallValue;

	/// Gateway to the functionality related to sending transactions from the current contract.
	fn send(&self) -> Self::SendApi;

	/// Gateway blockchain info related to the current transaction and to accounts.
	fn blockchain(&self) -> Self::BlockchainApi;

	/// Stateless crypto functions provided by the Arwen VM.
	fn crypto(&self) -> Self::CryptoApi;

	/// Gateway into the lower-level event log functionality.
	/// Gets called in auto-generated
	/// Using it directly is not recommended.
	/// TODO: consider moving to `ContractPrivateApi`.
	fn log_api_raw(&self) -> Self::LogApi;

	/// Currently for some auto-generated code involving callbacks.
	/// Please avoid using it directly.
	/// TODO: find a way to hide this API.
	fn error_api(&self) -> Self::ErrorApi;

	fn proxy<P: ProxyObjApi<SendApi = Self::SendApi>>(&self, address: Address) -> P {
		P::new_proxy_obj(self.send(), address)
	}
}

pub trait ContractPrivateApi {
	type ArgumentApi: EndpointArgumentApi + Clone + 'static;

	type FinishApi: EndpointFinishApi + ErrorApi + Clone + 'static;

	fn argument_api(&self) -> Self::ArgumentApi;

	fn finish_api(&self) -> Self::FinishApi;
}
