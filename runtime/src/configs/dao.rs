use crate::*;

parameter_types! {
	pub const DaoMotionDuration: BlockNumber = 7 * DAYS;
	pub const MinVetos: u32 = 3;
}

impl pallet_dao::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CouncilOrigin = EnsureRootOrCouncilApproval;
	type Proposal = RuntimeCall;
	type MotionDuration = DaoMotionDuration;
	type Tfgrid = GridStore;
	type NodeChanged = NodeChanged;
	type WeightInfo = pallet_dao::weights::SubstrateWeight<Runtime>;
	type MinVetos = MinVetos;
}
