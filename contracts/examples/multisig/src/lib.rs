#![no_std]

mod action;
mod user_role;

use action::Action;
use user_role::UserRole;

imports!();

#[cfg(feature = "elrond-wasm-module-users-default")]
use elrond_wasm_module_users_default::*;
#[cfg(feature = "elrond-wasm-module-users-wasm")]
use elrond_wasm_module_users_wasm::*;

#[elrond_wasm_derive::contract(MultisigImpl)]
pub trait Multisig {
	#[module(UsersModuleImpl)]
	fn users_module(&self) -> UsersModuleImpl<T, BigInt, BigUint>;

	#[view(getQuorum)]
	#[storage_get("quorum")]
	fn get_quorum(&self) -> usize;

	#[storage_set("quorum")]
	fn set_quorum(&self, quorum: usize);

	#[view(getUserRole)]
	#[storage_get("user_role")]
	fn get_user_id_to_role(&self, user_id: usize) -> UserRole;

	#[storage_set("user_role")]
	fn set_user_id_to_role(&self, user_id: usize, user_role: UserRole);

	#[view(getBoardSize)]
	#[storage_get("board_size")]
	fn get_board_size(&self) -> usize;

	#[storage_set("board_size")]
	fn set_board_size(&self, board_size: usize);

	#[view(getActionLastIndex)]
	#[storage_get("action_last_index")]
	fn get_action_last_index(&self) -> usize;

	#[storage_set("action_last_index")]
	fn set_action_last_index(&self, action_last_index: usize);

	#[view(getPendingActionCount)]
	#[storage_get("pending_action_count")]
	fn get_pending_action_count(&self) -> usize;

	#[storage_set("pending_action_count")]
	fn set_pending_action_count(&self, pending_action_count: usize);

	#[view(getActionData)]
	#[storage_get("action_data")]
	fn get_action_data(&self, action_id: usize) -> Action<BigUint>;

	#[storage_set("action_data")]
	fn set_action_data(&self, action_id: usize, action_data: &Action<BigUint>);

	#[storage_get("action_signer_ids")]
	fn get_action_signer_ids(&self, action_id: usize) -> Vec<usize>;

	#[storage_set("action_signer_ids")]
	fn set_action_signer_ids(&self, action_id: usize, action_signer_ids: &[usize]);

	#[init]
	fn init(&self, quorum: usize, #[var_args] board: VarArgs<Address>) -> SCResult<()> {
		require!(quorum <= board.len(), "quorum cannot exceed board size");

		self.set_quorum(quorum);

		for (i, address) in board.iter().enumerate() {
			let user_id = i + 1;
			self.users_module().set_user_id(&address, user_id);
			self.users_module().set_user_address(user_id, &address);
			self.set_user_id_to_role(user_id, UserRole::BoardMember);
		}
		self.users_module().set_num_users(board.len());
		self.set_board_size(board.len());

		Ok(())
	}

	#[payable]
	#[endpoint]
	fn deposit(&self) {}

	fn propose_action(&self, action: Action<BigUint>) -> SCResult<usize> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(
			caller_role.can_propose(),
			"only board members and proposers can propose"
		);

		let action_id = self.get_action_last_index() + 1;
		self.set_action_last_index(action_id);

		self.set_pending_action_count(self.get_pending_action_count() + 1);

		self.set_action_data(action_id, &action);

		if caller_role.can_sign() {
			// also sign
			// since the action is newly created, the caller can be the only signer
			self.set_action_signer_ids(action_id, &[caller_id][..]);
		}

		Ok(action_id)
	}

	#[endpoint(proposeAddBoardMemeber)]
	fn propose_add_board_member(&self, board_member_address: Address) -> SCResult<usize> {
		self.propose_action(Action::AddBoardMember(board_member_address))
	}

	#[endpoint(proposeAddProposer)]
	fn propose_add_proposer(&self, proposer_address: Address) -> SCResult<usize> {
		self.propose_action(Action::AddProposer(proposer_address))
	}

	/// Removes user regardless of whether it is a board member or proposer.
	#[endpoint(proposeRemoveUser)]
	fn propose_remove_user(&self, user_address: Address) -> SCResult<usize> {
		self.propose_action(Action::RemoveUser(user_address))
	}

	#[endpoint(proposeChangeQuorum)]
	fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
		self.propose_action(Action::ChangeQuorum(new_quorum))
	}

	#[endpoint(proposeSendEgld)]
	fn propose_send_egld(&self, to: Address, amount: BigUint) -> SCResult<usize> {
		self.propose_action(Action::SendEgld(to, amount))
	}

	#[view]
	fn signed(&self, user: Address, action_id: usize) -> bool {
		let user_id = self.users_module().get_user_id(&user);
		if user_id == 0 {
			false
		} else {
			let signer_ids = self.get_action_signer_ids(action_id);
			signer_ids.contains(&user_id)
		}
	}

	#[view(userRole)]
	fn user_role(&self, user: Address) -> UserRole {
		let user_id = self.users_module().get_user_id(&user);
		if user_id == 0 {
			UserRole::None
		} else {
			self.get_user_id_to_role(user_id)
		}
	}

	#[endpoint]
	fn sign(&self, action_id: usize) -> SCResult<()> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(caller_role.can_sign(), "only board members can sign");

		let mut signer_ids = self.get_action_signer_ids(action_id);
		if !signer_ids.contains(&caller_id) {
			signer_ids.push(caller_id);
			self.set_action_signer_ids(action_id, signer_ids.as_slice());
		}

		Ok(())
	}

	#[endpoint]
	fn unsign(&self, action_id: usize) -> SCResult<()> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(caller_role.can_sign(), "only board members can un-sign");

		let mut signer_ids = self.get_action_signer_ids(action_id);
		if let Some(signer_pos) = signer_ids
			.iter()
			.position(|signer_id| *signer_id == caller_id)
		{
			// since we don't care about the order,
			// it is ok to call swap_remove, which is O(1)
			signer_ids.swap_remove(signer_pos);
			self.set_action_signer_ids(action_id, signer_ids.as_slice());

			if signer_ids.is_empty() {
				// last signer withdrew the signature, time to clean up
				self.set_action_data(action_id, &Action::Nothing);
			}
		}

		Ok(())
	}

	/// Can be used to:
	/// - create new user (board member / proposer)
	/// - remove user (board member / proposer)
	/// - reactivate removed user
	/// - convert between board member and proposer
	/// Will keep the board size in sync.
	fn change_user_role(&self, user_address: Address, new_role: UserRole) {
		let user_id = self.users_module().get_or_create_user(&user_address);
		let old_role = if user_id == 0 {
			UserRole::None
		} else {
			self.get_user_id_to_role(user_id)
		};
		self.set_user_id_to_role(user_id, new_role);
		if old_role == UserRole::BoardMember {
			if new_role != UserRole::BoardMember {
				self.set_board_size(self.get_board_size() - 1);
			}
		} else {
			if new_role == UserRole::BoardMember {
				self.set_board_size(self.get_board_size() + 1);
			}
		}
	}

	#[endpoint(performAction)]
	fn perform_action(&self, action_id: usize) -> SCResult<()> {
		let caller_address = self.get_caller();
		let caller_id = self.users_module().get_user_id(&caller_address);
		let caller_role = self.get_user_id_to_role(caller_id);
		require!(
			caller_role.can_propose(),
			"only board members and proposers can perform actions"
		);

		let quorum = self.get_quorum();
		let signer_ids = self.get_action_signer_ids(action_id);
		let valid_signers_count = signer_ids
			.iter()
			.filter(|signer_id| {
				// it is possible for signers to lose their role
				// they are not automatically removed from all actions when doing so
				// this the contract needs to re-check every time when actions are performed
				let signer_role = self.get_user_id_to_role(**signer_id);
				signer_role.can_sign()
			})
			.count();
		require!(valid_signers_count >= quorum, "quorum has not been reached");

		let action = self.get_action_data(action_id);
		match action {
			Action::Nothing => {},
			Action::AddBoardMember(board_member_address) => {
				self.change_user_role(board_member_address, UserRole::BoardMember);
			},
			Action::AddProposer(proposer_address) => {
				self.change_user_role(proposer_address, UserRole::Proposer);
			},
			Action::RemoveUser(user_address) => {
				self.change_user_role(user_address, UserRole::None);
			},
			Action::ChangeQuorum(new_quorum) => {
				require!(
					new_quorum <= self.get_board_size(),
					"quorum cannot exceed board size"
				);
				self.set_quorum(new_quorum)
			},
			Action::SendEgld(to, amount) => {
				self.send_tx(&to, &amount, "");
			},
		}

		// clean up storage
		self.set_action_data(action_id, &Action::Nothing);
		self.set_action_signer_ids(action_id, &[][..]);
		Ok(())
	}
}