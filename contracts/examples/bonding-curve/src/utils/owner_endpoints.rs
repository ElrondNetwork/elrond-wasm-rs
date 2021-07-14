elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::function_selector::FunctionSelector;
use crate::utils::structs::BondingCurve;
use crate::utils::{events, storage, structs::Token};

use super::structs::{CurveArguments, SupplyType};

#[elrond_wasm_derive::module]
pub trait OwnerEndpointsModule: storage::StorageModule + events::EventsModule {
	#[endpoint(setLocalRoles)]
	fn set_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.set_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
	}

	#[endpoint(unsetLocalRoles)]
	fn unset_local_roles(
		&self,
		address: Address,
		token_identifier: TokenIdentifier,
		#[var_args] roles: VarArgs<EsdtLocalRole>,
	) -> AsyncCall<Self::SendApi> {
		ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
			.unset_special_roles(&address, &token_identifier, roles.as_slice())
			.async_call()
			.with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
	}

	#[callback]
	fn change_roles_callback(&self, #[call_result] result: AsyncCallResult<()>) -> SCResult<()> {
		match result {
			AsyncCallResult::Ok(()) => Ok(()),
			AsyncCallResult::Err(message) => Err(message.err_msg.into()),
		}
	}

	#[endpoint(setBondingCurve)]
	fn set_bonding_curve(
		&self,
		token: Token,
		function: FunctionSelector<Self::BigUint>,
	) -> SCResult<()> {
		require!(
			!self.bonding_curve(&token).is_empty(),
			"Token is not issued yet!"
		);
		self.bonding_curve(&token)
			.update(|bonding_curve| bonding_curve.curve = function);
		Ok(())
	}

	#[endpoint(deposit)]
	#[payable("*")]
	fn deposit(
		&self,
		#[payment] amount: Self::BigUint,
		#[payment_token] identifier: TokenIdentifier,
		#[payment_nonce] nonce: u64,
		#[var_args] supply_type: OptionalArg<SupplyType<Self::BigUint>>,
		#[var_args] payment: OptionalArg<TokenIdentifier>,
	) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let mut deposited_tokens = Vec::new();
		if self.owned_tokens(&caller).is_empty() {
			deposited_tokens.push(identifier.clone());
		} else {
			deposited_tokens = self.owned_tokens(&caller).get();
		}
		let token_type = self.call_value().esdt_token_type();

		require!(
			token_type != EsdtTokenType::SemiFungible || nonce != 0,
			"Nonce should not be 0!"
		);
		let mut token = Token { identifier, nonce };
		let mut set_payment = TokenIdentifier::egld();
		let mut set_supply = SupplyType::Unlimited;
		if self.bonding_curve(&token).is_empty() {
			set_payment = payment
				.into_option()
				.ok_or("Expected provided accepted_payment for the token")?;
			set_supply = supply_type
				.into_option()
				.ok_or("Expected provided supply_type for the token")?;
		}

		if self.token_details(&token.identifier).is_empty() {
			self.token_details(&token.identifier)
				.set(&(token_type.clone(), caller.clone()))
		} else {
			let (_, stored_caller) = self.token_details(&token.identifier).get();
			require!(
				stored_caller == caller,
				"The token was alreade deposited by another address"
			);
		}

		if token_type == EsdtTokenType::Fungible || token_type == EsdtTokenType::NonFungible {
			token.nonce = 0u64;
		}

		self.store_bonding_curve(token, amount, set_supply, set_payment)?;
		self.owned_tokens(&caller).set(&deposited_tokens);
		Ok(())
	}

	#[endpoint(claim)]
	fn claim(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let owned_tokens = self.owned_tokens(&caller).get();

		for token in owned_tokens {
			let (token_type, min_loop_nonce, max_loop_nonce) = self.get_token_nonce_ranges(&token);

			for current_check_nonce in min_loop_nonce..=max_loop_nonce {
				let bonding_curve = self
					.bonding_curve(&Token {
						identifier: token.clone(),
						nonce: current_check_nonce,
					})
					.get();

				self.send_token(
					&caller,
					token_type.clone(),
					bonding_curve.arguments.balance,
					&token,
					current_check_nonce,
				);
				self.send_token(
					&caller,
					token_type.clone(),
					bonding_curve.payment_amount,
					&bonding_curve.payment_token,
					0u64,
				);
				self.bonding_curve(&Token {
					identifier: token.clone(),
					nonce: current_check_nonce,
				})
				.clear();
			}
			self.token_details(&token).clear();
		}
		Ok(())
	}

	fn send_token(
		&self,
		to: &Address,
		token_type: EsdtTokenType,
		amount: Self::BigUint,
		token: &TokenIdentifier,
		nonce: u64,
	) {
		match token_type {
			EsdtTokenType::Fungible => {
				self.send().direct(to, token, &amount, &[]);
			},
			EsdtTokenType::NonFungible | EsdtTokenType::SemiFungible => {
				self.send().direct_nft(to, token, nonce, &amount, &[]);
			},
			EsdtTokenType::Invalid => {},
		}
	}
	fn get_current_nonce(&self, identifier: &TokenIdentifier) -> u64 {
		self.blockchain()
			.get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), identifier)
	}

	fn check_supply(&self, token: &Token, amount: &Self::BigUint) -> SCResult<()> {
		let bonding_curve = self.bonding_curve(token).get();

		if bonding_curve.arguments.supply_type != SupplyType::Unlimited {
			require!(
				bonding_curve.arguments.available_supply
					< bonding_curve.arguments.supply_type.get_limit()?,
				"Maximum supply limit reached!"
			);

			require!(
				bonding_curve.arguments.available_supply + amount.clone()
					<= bonding_curve.arguments.supply_type.get_limit()?,
				"Minting will exceed the maximum supply limit!"
			);
		}
		Ok(())
	}

	fn store_bonding_curve(
		&self,
		token: Token,
		amount: Self::BigUint,
		supply_type: SupplyType<Self::BigUint>,
		payment: TokenIdentifier,
	) -> SCResult<()> {
		let mut curve = FunctionSelector::None;
		let mut arguments;
		let payment_token;
		let payment_amount: Self::BigUint;

		if self.bonding_curve(&token).is_empty() {
			arguments = CurveArguments {
				supply_type,
				available_supply: amount.clone(),
				balance: amount,
			};
			payment_token = payment;
			payment_amount = 0u64.into();
		} else {
			self.check_supply(&token, &amount)?;
			let bonding_curve = self.bonding_curve(&token).get();
			payment_token = bonding_curve.payment_token;
			payment_amount = bonding_curve.payment_amount;
			curve = bonding_curve.curve;
			arguments = bonding_curve.arguments;
			arguments.balance += &amount;
			arguments.available_supply += amount;
		}
		self.bonding_curve(&token).set(&BondingCurve {
			curve,
			arguments,
			payment_token,
			payment_amount,
		});
		Ok(())
	}
}
