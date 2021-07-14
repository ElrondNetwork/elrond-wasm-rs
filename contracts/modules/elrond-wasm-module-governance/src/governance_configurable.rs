elrond_wasm::imports!();

#[allow(clippy::too_many_arguments)]
#[elrond_wasm_derive::module]
pub trait GovernanceConfigurablePropertiesModule {
	// endpoints - owner-only

	/// The module can't protect its storage from the main SC, so it's the developers responsibility
	/// to not modify parameters manually
	#[endpoint(initGovernanceModule)]
	fn init_governance_module(
		&self,
		governance_token_id: TokenIdentifier,
		quorum: Self::BigUint,
		min_token_balance_for_proposal: Self::BigUint,
		max_actions_per_proposal: usize,
		voting_delay_in_blocks: u64,
		voting_period_in_blocks: u64,
		lock_time_after_voting_ends_in_blocks: u64,
	) -> SCResult<()> {
		only_owner!(self, "Only owner may initialize governance module");
		require!(
			governance_token_id.is_valid_esdt_identifier(),
			"Invalid ESDT token ID provided for governance_token_id"
		);

		self.governance_token_id()
			.set_if_empty(&governance_token_id);

		self.try_change_quorum(quorum)?;
		self.try_change_min_token_balance_for_proposing(min_token_balance_for_proposal)?;
		self.try_change_max_actions_per_proposal(max_actions_per_proposal)?;
		self.try_change_voting_delay_in_blocks(voting_delay_in_blocks)?;
		self.try_change_voting_period_in_blocks(voting_period_in_blocks)?;
		self.try_change_lock_time_after_voting_ends_in_blocks(
			lock_time_after_voting_ends_in_blocks,
		)?;

		Ok(())
	}

	// endpoints - these can only be called by the SC itself.
	// i.e. only by proposing and executing an action with the SC as dest and the respective func name

	#[endpoint(changeQuorum)]
	fn change_quorum(&self, new_value: Self::BigUint) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_quorum(new_value)?;

		Ok(())
	}

	#[endpoint(changeMinTokenBalanceForProposing)]
	fn change_min_token_balance_for_proposing(&self, new_value: Self::BigUint) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_min_token_balance_for_proposing(new_value)?;

		Ok(())
	}

	#[endpoint(changeMaxActionsPerProposal)]
	fn change_max_actions_per_proposal(&self, new_value: usize) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_max_actions_per_proposal(new_value)?;

		Ok(())
	}

	#[endpoint(changeVotingDelayInBlocks)]
	fn change_voting_delay_in_blocks(&self, new_value: u64) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_voting_delay_in_blocks(new_value)?;

		Ok(())
	}

	#[endpoint(changeVotingPeriodInBlocks)]
	fn change_voting_period_in_blocks(&self, new_value: u64) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_voting_period_in_blocks(new_value)?;

		Ok(())
	}

	#[endpoint(changeLockTimeAfterVotingEndsInBlocks)]
	fn change_lock_time_after_voting_ends_in_blocks(&self, new_value: u64) -> SCResult<()> {
		self.require_caller_self()?;

		self.try_change_lock_time_after_voting_ends_in_blocks(new_value)?;

		Ok(())
	}

	// private

	fn require_caller_self(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let sc_address = self.blockchain().get_sc_address();

		require!(
			caller == sc_address,
			"Only the SC itself may call this function"
		);

		Ok(())
	}

	fn try_change_quorum(&self, new_value: Self::BigUint) -> SCResult<()> {
		require!(new_value != 0, "Quorum can't be set to 0");

		self.quorum().set(&new_value);

		Ok(())
	}

	fn try_change_min_token_balance_for_proposing(&self, new_value: Self::BigUint) -> SCResult<()> {
		require!(
			new_value != 0,
			"Min token balance for proposing can't be set to 0"
		);

		self.min_token_balance_for_proposing().set(&new_value);

		Ok(())
	}

	fn try_change_max_actions_per_proposal(&self, new_value: usize) -> SCResult<()> {
		require!(new_value != 0, "Max actions per proposal can't be set to 0");

		self.max_actions_per_proposal().set(&new_value);

		Ok(())
	}

	fn try_change_voting_delay_in_blocks(&self, new_value: u64) -> SCResult<()> {
		require!(new_value != 0, "Voting delay in blocks can't be set to 0");

		self.voting_delay_in_blocks().set(&new_value);

		Ok(())
	}

	fn try_change_voting_period_in_blocks(&self, new_value: u64) -> SCResult<()> {
		require!(
			new_value != 0,
			"Voting period (in blocks) can't be set to 0"
		);

		self.voting_period_in_blocks().set(&new_value);

		Ok(())
	}

	fn try_change_lock_time_after_voting_ends_in_blocks(&self, new_value: u64) -> SCResult<()> {
		require!(
			new_value != 0,
			"Lock time after voting ends (in blocks) can't be set to 0"
		);

		self.lock_time_after_voting_ends_in_blocks().set(&new_value);

		Ok(())
	}

	// storage - fixed parameters

	#[view(getGovernanceTokenId)]
	#[storage_mapper("governance:governanceTokenId")]
	fn governance_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	// storage - configurable parameters

	#[view(getQuorum)]
	#[storage_mapper("governance:quorum")]
	fn quorum(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getMinTokenBalanceForProposing)]
	#[storage_mapper("governance:minTokenBalanceForProposing")]
	fn min_token_balance_for_proposing(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getMaxActionsPerProposal)]
	#[storage_mapper("governance:maxActionsPerProposal")]
	fn max_actions_per_proposal(&self) -> SingleValueMapper<Self::Storage, usize>;

	#[view(getVotingDelayInBlocks)]
	#[storage_mapper("governance:votingDelayInBlocks")]
	fn voting_delay_in_blocks(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getVotingPeriodInBlocks)]
	#[storage_mapper("governance:votingPeriodInBlocks")]
	fn voting_period_in_blocks(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getLockTimeAfterVotingEndsInBlocks)]
	#[storage_mapper("governance:lockTimeAfterVotingEndsInBlocks")]
	fn lock_time_after_voting_ends_in_blocks(&self) -> SingleValueMapper<Self::Storage, u64>;
}
