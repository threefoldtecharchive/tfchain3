use crate::*;

impl pallet_tft_price::Config for Runtime {
	type AuthorityId = pallet_tft_price::AuthId;
	type Call = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RestrictedOrigin = EnsureRootOrCouncilApproval;
	type FindNextAuthor = FindNextAuraAuthor;
}
