use crate::*;

pub type Serial = pallet_tfgrid::pallet::SerialNumberOf<Runtime>;
pub type Loc = pallet_tfgrid::pallet::LocationOf<Runtime>;
pub type Interface = pallet_tfgrid::pallet::InterfaceOf<Runtime>;

pub type TfgridNode = pallet_tfgrid::pallet::TfgridNode<Runtime>;

pub struct NodeChanged;
impl ChangeNode<Loc, Interface, Serial> for NodeChanged {
	fn node_changed(old_node: Option<&TfgridNode>, new_node: &TfgridNode) {
		Dao::node_changed(old_node, new_node)
	}

	fn node_deleted(node: &TfgridNode) {
		GridContracts::node_deleted(node);
		Dao::node_deleted(node);
	}
}

pub struct PublicIpModifierType;
impl PublicIpModifier for PublicIpModifierType {
	fn ip_removed(ip: &PublicIP) {
		GridContracts::ip_removed(ip);
	}
}

parameter_types! {
	pub const MaxFarmNameLength: u32 = 40;
	pub const MaxInterfaceIpsLength: u32 = 10;
	pub const MaxInterfacesLength: u32 = 10;
	pub const MaxFarmPublicIps: u32 = 512;
}

impl pallet_tfgrid::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RestrictedOrigin = EnsureRootOrCouncilApproval;
	type WeightInfo = pallet_tfgrid::weights::SubstrateWeight<Runtime>;
	type NodeChanged = NodeChanged;
	type PublicIpModifier = PublicIpModifierType;
	type TermsAndConditions = pallet_tfgrid::terms_cond::TermsAndConditions<Runtime>;
	type MaxFarmNameLength = MaxFarmNameLength;
	type MaxFarmPublicIps = MaxFarmPublicIps;
	type FarmName = pallet_tfgrid::farm::FarmName<Runtime>;
	type MaxInterfacesLength = MaxInterfacesLength;
	type InterfaceName = pallet_tfgrid::interface::InterfaceName<Runtime>;
	type InterfaceMac = pallet_tfgrid::interface::InterfaceMac<Runtime>;
	type InterfaceIP = pallet_tfgrid::interface::InterfaceIp<Runtime>;
	type MaxInterfaceIpsLength = MaxInterfaceIpsLength;
	type CountryName = pallet_tfgrid::node::CountryName<Runtime>;
	type CityName = pallet_tfgrid::node::CityName<Runtime>;
	type Location = pallet_tfgrid::node::Location<Runtime>;
	type SerialNumber = pallet_tfgrid::node::SerialNumber<Runtime>;
}
