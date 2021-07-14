elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::utils::{
	events, owner_endpoints, storage,
	structs::{SupplyType, Token},
};

#[elrond_wasm_derive::module]
pub trait TokenMethods:
	storage::StorageModule + events::EventsModule + owner_endpoints::OwnerEndpointsModule
{
	#[endpoint(ftLocalMint)]
	fn local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) -> SCResult<()> {
		self.check_supply(
			&Token {
				identifier: token_identifier.clone(),
				nonce: 0u64,
			},
			&amount,
		)?;
		self.send().esdt_local_mint(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += &amount;
		});
		Ok(())
	}

	#[endpoint(ftLocalBurn)]
	fn local_burn(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
		self.send().esdt_local_burn(&token_identifier, &amount);
		self.bonding_curve(&Token {
			identifier: token_identifier,
			nonce: 0u64,
		})
		.update(|bonding_curve| {
			bonding_curve.arguments.available_supply -= &amount;
			bonding_curve.arguments.balance -= &amount;
		});
	}

	#[endpoint(nftCreate)]
	#[allow(clippy::too_many_arguments)]
	fn create(
		&self,
		identifier: TokenIdentifier,
		amount: Self::BigUint,
		name: BoxedBytes,
		royalties: Self::BigUint,
		hash: BoxedBytes,
		attributes: BoxedBytes,
		uri: BoxedBytes,
		#[var_args] supply_type: OptionalArg<SupplyType<Self::BigUint>>,
		#[var_args] payment: OptionalArg<TokenIdentifier>,
	) -> SCResult<()> {
		self.send().esdt_nft_create(
			&identifier,
			&amount,
			&name,
			&royalties,
			&hash,
			&attributes,
			&[uri],
		);

		let nonce = self.get_current_nonce(&identifier);
		let (token_type, _) = self.token_details(&identifier).get();
		require!(
			!self.token_details(&identifier).is_empty(),
			"Token not issued"
		);

		let mut token = Token { identifier, nonce };

		if token_type == EsdtTokenType::Fungible || token_type == EsdtTokenType::NonFungible {
			token.nonce = 0u64;
		}

		require!(
			token_type != EsdtTokenType::SemiFungible || token.nonce != 0,
			"Nonce should not be 0!"
		);

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
		self.store_bonding_curve(token, amount, set_supply, set_payment)?;
		Ok(())
	}

	#[endpoint(nftBurn)]
	fn burn(&self, identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) -> SCResult<()> {
		self.send().esdt_nft_burn(&identifier, nonce, &amount);

		if self.call_value().esdt_token_type() == EsdtTokenType::SemiFungible {
			let token = &Token { identifier, nonce };

			if self.bonding_curve(token).is_empty() {
				return Err("Token has not been created.".into());
			}
			let mut bonding_curve = self.bonding_curve(token).get();
			bonding_curve.arguments.balance += &amount;
			bonding_curve.arguments.available_supply += &amount;
			self.bonding_curve(token).set(&bonding_curve);
		} else {
			let token = &Token {
				identifier,
				nonce: 0u64,
			};

			if self.bonding_curve(token).is_empty() {
				return Err("Token has not been created.".into());
			}
			self.bonding_curve(token).clear();
		}
		Ok(())
	}

	#[endpoint(sftAddQuantity)]
	fn add_quantity(
		&self,
		identifier: TokenIdentifier,
		nonce: u64,
		amount: Self::BigUint,
	) -> SCResult<()> {
		let token = Token { identifier, nonce };
		self.check_supply(&token, &amount)?;
		self.send()
			.esdt_nft_add_quantity(&token.identifier, nonce, &amount);

		self.bonding_curve(&token).update(|bonding_curve| {
			bonding_curve.arguments.available_supply += &amount;
			bonding_curve.arguments.balance += amount;
		});
		Ok(())
	}
}
