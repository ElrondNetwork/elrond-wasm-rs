use crate::function_selector::FunctionSelector;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum SupplyType<BigUint>
where
	BigUint: BigUintApi,
{
	Limited(BigUint),
	Unlimited,
}

impl<BigUint> SupplyType<BigUint>
where
	BigUint: BigUintApi,
{
	pub fn get_limit(&self) -> SCResult<BigUint> {
		match &self {
			SupplyType::Limited(limit) => Ok(limit.clone()),
			SupplyType::Unlimited => Err("Unlimited has no limit!".into()),
		}
	}
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct CurveArguments<BigUint: BigUintApi> {
	pub supply_type: SupplyType<BigUint>,
	pub available_supply: BigUint,
	pub balance: BigUint,
}

impl<BigUint> CurveArguments<BigUint>
where
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	BigUint: BigUintApi,
{
	pub fn first_token_available(&self) -> BigUint {
		&self.available_supply - &self.balance
	}
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct Token {
	pub identifier: TokenIdentifier,
	pub nonce: u64,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct BondingCurve<BigUint: BigUintApi> {
	pub curve: FunctionSelector<BigUint>,
	pub arguments: CurveArguments<BigUint>,
	pub payment_token: TokenIdentifier,
	pub payment_amount: BigUint,
}
