#![no_std]
#![allow(unused_attributes)]

use elrond_wasm::{require, sc_error};
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod deposit_info;
use deposit_info::DepositInfo;

pub const SECONDS_PER_ROUND: u64 = 6;

#[elrond_wasm_derive::contract(DigitalCashImpl)]
pub trait DigitalCash {
    #[init]
    fn init(&self) {}

    fn get_expiration_round(&self, valability: u64) -> u64 {
        let valability_rounds = valability / SECONDS_PER_ROUND;

        return self.get_block_round() + valability_rounds;
    }

    #[endpoint]
    #[payable("*")]
    fn fund(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
        address: Address,
        valability: u64,
    ) -> SCResult<()> {
        require!(payment > 0, "amount must be greater than 0");
        require!(self.deposit(&address).is_empty(), "key already used");

        let nft_nonce = self.call_value().esdt_token_nonce();

        let deposit = &DepositInfo {
            amount: payment,
            depositor_address: self.get_caller(),
            expiration_round: self.get_expiration_round(valability),
            token_name: token,
            nonce: nft_nonce,
        };

        self.deposit(&address).set(deposit);

        Ok(())
    }

    #[endpoint]
    fn withdraw(&self, address: Address) -> SCResult<()> {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();

        require!(
            deposit.expiration_round < self.get_block_round(),
            "withdrawal has not been available yet"
        );
        if deposit.nonce != 0 {
            self.send().direct_esdt_nft_via_transfer_exec(
                &deposit.depositor_address,
                &deposit.token_name.as_esdt_identifier(),
                deposit.nonce,
                &deposit.amount,
                b"successful withdrawal",
            );
        } else {
            self.send().direct(
                &deposit.depositor_address,
                &deposit.token_name,
                &deposit.amount,
                b"successful withdrawal",
            );
            self.deposit(&address).clear();
        };

        Ok(())
    }

    #[endpoint]
    fn claim(&self, address: Address, signature: &[u8]) -> SCResult<()> {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let deposit = self.deposit(&address).get();
        let caller_address: Address = self.get_caller();

        require!(
            deposit.expiration_round >= self.get_block_round(),
            "deposit expired"
        );
        require!(
            self.verify_ed25519(address.as_bytes(), caller_address.as_bytes(), signature),
            "invalid signature"
        );

        if deposit.nonce != 0 {
            self.send().direct_esdt_nft_via_transfer_exec(
                &self.get_caller(),
                &deposit.token_name.as_esdt_identifier(),
                deposit.nonce,
                &deposit.amount,
                b"successful withdrawal",
            );
        } else {
            self.send().direct(
                &self.get_caller(),
                &deposit.token_name,
                &deposit.amount,
                b"successful claim",
            );
        }
        self.deposit(&address).clear();

        Ok(())
    }

    #[view(amount)]
    fn get_amount(&self, address: Address) -> SCResult<BigUint> {
        require!(!self.deposit(&address).is_empty(), "non-existent key");

        let data = self.deposit(&address).get();

        Ok(data.amount)
    }

    //storage

    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &Address) -> SingleValueMapper<Self::Storage, DepositInfo<BigUint>>;
}

