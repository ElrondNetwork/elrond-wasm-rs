elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::curves::curve_function::CurveFunction;
use crate::function_selector::FunctionSelector;
use crate::utils::{
	events, storage,
	structs::{CurveArguments, Token},
};

#[elrond_wasm_derive::module]
pub trait UserEndpointsModule: storage::StorageModule + events::EventsModule {
	#[payable("*")]
	#[endpoint(sellToken)]
	fn sell_token(
		&self,
		#[payment_amount] sell_amount: Self::BigUint,
		#[payment_token] offered_token: TokenIdentifier,
		#[payment_nonce] nonce: u64,
	) -> SCResult<()> {
		let owned_token = Token {
			identifier: offered_token,
			nonce,
		};
		self.check_sell_requirements(&owned_token, &sell_amount)?;

		let calculated_price = self.bonding_curve(&owned_token).update(|bonding_curve| {
			let price = self.compute_sell_price(
				&bonding_curve.curve,
				sell_amount.clone(),
				bonding_curve.arguments.clone(),
			);
			bonding_curve.payment_amount -= price.clone()?;
			bonding_curve.arguments.balance += sell_amount;
			price
		})?;

		let caller = self.blockchain().get_caller();

		self.send().direct(
			&caller,
			&self.bonding_curve(&owned_token).get().payment_token,
			&calculated_price,
			b"selling",
		);

		self.sell_token_event(&caller, &calculated_price);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(buyToken)]
	fn buy_token(
		&self,
		#[payment_amount] payment: Self::BigUint,
		#[payment_token] offered_token: TokenIdentifier,
		requested_amount: Self::BigUint,
		requested_token: TokenIdentifier,
		#[var_args] requested_nonce: OptionalArg<u64>,
	) -> SCResult<()> {
		let exchanging_token = Token {
			identifier: offered_token,
			nonce: 0u64,
		};
		let (token_type, _) = self.token_details(&requested_token).get();
		let mut desired_nonce = 0u64;
		let owned_token;

		if token_type != EsdtTokenType::Fungible {
			desired_nonce = requested_nonce
				.into_option()
				.ok_or("Expected nonce for the desired NFT")?;
		}
		if token_type == EsdtTokenType::SemiFungible {
			owned_token = Token {
				identifier: requested_token,
				nonce: desired_nonce,
			};
		} else {
			owned_token = Token {
				identifier: requested_token,
				nonce: 0u64,
			};
		}
		self.check_buy_requirements(&owned_token, &exchanging_token, &requested_amount)?;

		let calculated_price = self.bonding_curve(&owned_token).update(|bonding_curve| {
			let price = self.compute_buy_price(
				&bonding_curve.curve,
				requested_amount.clone(),
				bonding_curve.arguments.clone(),
			);
			let price_clone = price.clone()?;
			require!(
				price_clone <= payment,
				"The payment provided is not enough for the transaction"
			);
			bonding_curve.payment_amount += price_clone;
			bonding_curve.arguments.balance -= &requested_amount;

			price
		})?;

		let caller = self.blockchain().get_caller();

		if token_type == EsdtTokenType::Fungible {
			self.send().direct(
				&caller,
				&owned_token.identifier,
				&requested_amount,
				b"buying",
			);
		} else {
			self.send().direct_nft(
				&caller,
				&(owned_token.identifier),
				desired_nonce,
				&requested_amount,
				b"buying",
			);
		}

		self.send().direct(
			&caller,
			&exchanging_token.identifier,
			&(&payment - &calculated_price),
			b"rest",
		);

		self.buy_token_event(&caller, &calculated_price);
		Ok(())
	}

	#[view]
	fn get_buy_price(
		&self,
		amount: Self::BigUint,
		identifier: TokenIdentifier,
		nonce: u64,
	) -> SCResult<Self::BigUint> {
		let token = Token { identifier, nonce };
		self.check_token_exists(&token)?;

		let bonding_curve = self.bonding_curve(&token).get();
		self.compute_buy_price(&bonding_curve.curve, amount, bonding_curve.arguments)
	}

	#[view]
	fn get_sell_price(
		&self,
		amount: Self::BigUint,
		identifier: TokenIdentifier,
		nonce: u64,
	) -> SCResult<Self::BigUint> {
		let token = Token { identifier, nonce };
		self.check_token_exists(&token)?;

		let bonding_curve = self.bonding_curve(&token).get();
		self.compute_sell_price(&bonding_curve.curve, amount, bonding_curve.arguments)
	}

	fn check_token_exists(&self, issued_token: &Token) -> SCResult<()> {
		require!(
			!self.bonding_curve(issued_token).is_empty(),
			"Token is not issued yet!"
		);

		Ok(())
	}

	#[view(getTokenAvailability)]
	fn get_token_availability(
		&self,
		identifier: TokenIdentifier,
	) -> MultiResultVec<MultiResult2<u64, Self::BigUint>> {
		let (_, min_loop_nonce, max_loop_nonce) = self.get_token_nonce_ranges(&identifier);
		let mut availability = Vec::new();

		for current_check_nonce in min_loop_nonce..=max_loop_nonce {
			let bonding_curve = self
				.bonding_curve(&Token {
					identifier: identifier.clone(),
					nonce: current_check_nonce,
				})
				.get();
			availability.push(MultiArg2((
				current_check_nonce,
				bonding_curve.arguments.balance,
			)));
		}
		availability.into()
	}

	fn check_sell_requirements(
		&self,
		issued_token: &Token,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		self.check_token_exists(issued_token)?;

		let bonding_curve = self.bonding_curve(issued_token).get();

		require!(
			bonding_curve.curve != FunctionSelector::None,
			"The token price was not set yet!"
		);
		require!(
			amount > &Self::BigUint::zero(),
			"Must pay more than 0 tokens!"
		);
		Ok(())
	}

	fn check_buy_requirements(
		&self,
		owned_token: &Token,
		exchanging_token: &Token,
		amount: &Self::BigUint,
	) -> SCResult<()> {
		self.check_token_exists(owned_token)?;

		let bonding_curve = self.bonding_curve(owned_token).get();

		require!(
			bonding_curve.curve != FunctionSelector::None,
			"The token price was not set yet!"
		);

		require!(
			amount > &Self::BigUint::zero(),
			"Must buy more than 0 tokens!"
		);

		self.check_given_token(&bonding_curve.payment_token, &exchanging_token.identifier)
	}

	fn check_given_token(
		&self,
		accepted_token: &TokenIdentifier,
		given_token: &TokenIdentifier,
	) -> SCResult<()> {
		if given_token != accepted_token {
			return Err(SCError::from(BoxedBytes::from_concat(&[
				b"Only ",
				accepted_token.as_esdt_identifier(),
				b" tokens accepted",
			])));
		}
		Ok(())
	}

	fn compute_buy_price(
		&self,
		function_selector: &FunctionSelector<Self::BigUint>,
		amount: Self::BigUint,
		arguments: CurveArguments<Self::BigUint>,
	) -> SCResult<Self::BigUint> {
		let token_start = arguments.first_token_available();
		function_selector.calculate_price(&token_start, &amount, &arguments)
	}

	fn compute_sell_price(
		&self,
		function_selector: &FunctionSelector<Self::BigUint>,
		amount: Self::BigUint,
		arguments: CurveArguments<Self::BigUint>,
	) -> SCResult<Self::BigUint> {
		let token_start = &arguments.first_token_available() - &amount;
		function_selector.calculate_price(&token_start, &amount, &arguments)
	}
}
