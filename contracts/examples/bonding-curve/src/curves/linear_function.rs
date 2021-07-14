elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::curves::curve_function::CurveFunction;

use crate::utils::structs::CurveArguments;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub struct LinearFunction<BigUint: BigUintApi> {
	pub initial_price: BigUint,
	pub linear_coefficient: BigUint,
}

impl<BigUint> CurveFunction<BigUint> for LinearFunction<BigUint>
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
		_arguments: &CurveArguments<BigUint>,
	) -> SCResult<BigUint> {
		Ok(
			&self.linear_coefficient * &sum_interval(amount, token_start)
				+ &self.initial_price * amount,
		)
	}
}
fn sum_interval<BigUint: BigUintApi>(n: &BigUint, x: &BigUint) -> BigUint
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
	x * n + &(n - &BigUint::from(1u64)) * n / BigUint::from(2u64)
}
