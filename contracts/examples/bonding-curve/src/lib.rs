#![no_std]
#![allow(unused_attributes)]
#![feature(trait_alias)]
#![feature(destructuring_assignment)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod curves;
mod function_selector;
mod token_methods;
mod utils;
use crate::utils::{events, owner_endpoints, storage, user_endpoints};

#[elrond_wasm_derive::contract]
pub trait Contract:
	storage::StorageModule
	+ events::EventsModule
	+ token_methods::TokenMethods
	+ user_endpoints::UserEndpointsModule
	+ owner_endpoints::OwnerEndpointsModule
{
}
