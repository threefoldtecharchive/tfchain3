use crate::*;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 2 * HOURS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrCouncilApproval;
	type RemoveOrigin = EnsureRootOrCouncilApproval;
	type SwapOrigin = EnsureRootOrCouncilApproval;
	type ResetOrigin = EnsureRootOrCouncilApproval;
	type PrimeOrigin = EnsureRootOrCouncilApproval;
	type MembershipInitialized = Council;
	type MembershipChanged = MembershipChangedGroup;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

use frame_support::traits::ChangeMembers;

pub struct MembershipChangedGroup;
impl ChangeMembers<AccountId> for MembershipChangedGroup {
	fn change_members_sorted(
		_incoming: &[AccountId],
		_outgoing: &[AccountId],
		_sorted_new: &[AccountId],
	) {
		// Council::change_members_sorted(incoming, outgoing, sorted_new);
	}
}
