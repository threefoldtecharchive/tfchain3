use crate::*;

parameter_types! {
	pub const MaxAuthorities: u32  = 100;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}
