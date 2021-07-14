# Function Selector 

The `FunctionSelector` stores the pre-defined functions. For now the only one available is [Linear](linear.md).
Other fuctions such as `Power`, `Sigmoid` and `Logarithmic` will be added later once the math module is functional.

When setting the bonding curve by a predefined function one mush pay attention by the parameters requested by the certain function. All the predefined functions are available in the curves folder and are implementing the `CurveFunction` trait.

Custom functions can be defined by adding the name of it in `FunctionSelector`, followed by defining the behaviour in the implementation of `CurveFunction`, in the `match` contained by the `calculate_price` function.

```rust
pub enum FunctionSelector<BigUint>
where
	BigUint: BigUintApi,
{
	Linear(LinearFunction<BigUint>),
	CustomExample(BigUint),
	None,
}
```