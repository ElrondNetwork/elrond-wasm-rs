elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
	curves::{curve_function::CurveFunction, linear_function::LinearFunction},
	utils::structs::CurveArguments,
};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum FunctionSelector<BigUint>
where
	BigUint: BigUintApi,
{
	Linear(LinearFunction<BigUint>),
	CustomExample(BigUint),
	None,
}

impl<BigUint> CurveFunction<BigUint> for FunctionSelector<BigUint>
where
	for<'a, 'b> &'a BigUint: core::ops::Add<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Sub<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Mul<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: core::ops::Div<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: core::ops::AddAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::SubAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::MulAssign<&'b BigUint>,
	for<'b> BigUint: core::ops::DivAssign<&'b BigUint>,
	BigUint: BigUintApi,
{
	fn calculate_price(
		&self,
		token_start: &BigUint,
		amount: &BigUint,
		arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		match &self {
			FunctionSelector::Linear(linear_function) => {
				CurveFunction::calculate_price(linear_function, token_start, amount, arguments)
			},

			FunctionSelector::CustomExample(initial_cost) => {
				let sum = token_start + amount;
				let price = &(&sum * &sum * sum / BigUint::from(3u64))
					+ &arguments.balance + initial_cost.clone();
				Ok(price)
			},
			FunctionSelector::None => Err("Bonding Curve function is not assiged".into()),
		}
	}
}
