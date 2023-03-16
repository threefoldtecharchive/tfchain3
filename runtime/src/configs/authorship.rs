use crate::*;

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

pub struct AuraAccountAdapter;
use sp_runtime::ConsensusEngineId;

impl FindAuthor<AccountId> for AuraAccountAdapter {
	fn find_author<'a, I>(digests: I) -> Option<AccountId>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(index) = pallet_aura::Pallet::<Runtime>::find_author(digests) {
			let validator = pallet_session::Pallet::<Runtime>::validators()[index as usize].clone();
			Some(validator)
		} else {
			None
		}
	}
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = AuraAccountAdapter;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ());
}
