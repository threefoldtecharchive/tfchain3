use crate::*;

parameter_types! {
	pub StakingPoolAccount: AccountId = get_staking_pool_account();
	pub BillingFrequency: u64 = 600;
	pub BillingReferencePeriod: u64 = SECS_PER_HOUR;
	pub GracePeriod: u64 = (14 * DAYS).into();
	pub DistributionFrequency: u16 = 24;
	pub RetryInterval: u32 = 20;
	pub MaxNameContractNameLength: u32 = 64;
	pub MaxDeploymentDataLength: u32 = 512;
}

pub fn get_staking_pool_account() -> AccountId {
	// decoded public key from staking pool account 5CNposRewardAccount11111111111111111111111111FSU
	AccountId::from([
		13, 209, 209, 166, 229, 163, 90, 168, 199, 245, 229, 126, 30, 221, 12, 63, 189, 106, 191,
		46, 170, 142, 244, 37, 72, 152, 110, 84, 162, 86, 32, 0,
	])
}

impl pallet_grid_contracts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type StakingPoolAccount = StakingPoolAccount; // TODO -> To treasury?
	type BillingFrequency = BillingFrequency;
	type BillingReferencePeriod = BillingReferencePeriod;
	type DistributionFrequency = DistributionFrequency;
	type GracePeriod = GracePeriod;
	type WeightInfo = pallet_grid_contracts::weights::SubstrateWeight<Runtime>;
	type NodeChanged = NodeChanged;
	type PublicIpModifier = PublicIpModifierType;
	type AuthorityId = pallet_grid_contracts::crypto::AuthId;
	type Call = RuntimeCall;
	type MaxNameContractNameLength = MaxNameContractNameLength;
	type NameContractName = pallet_grid_contracts::name_contract::NameContractName<Runtime>;
	type RestrictedOrigin = EnsureRootOrCouncilApproval;
	type MaxDeploymentDataLength = MaxDeploymentDataLength;
	type MaxNodeContractPublicIps = MaxFarmPublicIps;
	type Burn = ();
	type FindNextAuthor = FindNextAuraAuthor;
}
