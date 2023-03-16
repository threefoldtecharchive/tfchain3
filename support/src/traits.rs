use crate::types::PublicIP;
use frame_system::offchain::Signer;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_std::fmt::Error;

pub trait Tfgrid<AccountId, Name> {
	fn get_farm(farm_id: u32) -> Option<super::types::Farm<Name>>;
	fn is_farm_owner(farm_id: u32, who: AccountId) -> bool;
	fn is_twin_owner(twin_id: u32, who: AccountId) -> bool;
}

pub trait ChangeNode<Loc, If, Serial> {
	fn node_changed(
		node: Option<&super::types::Node<Loc, If, Serial>>,
		new_node: &super::types::Node<Loc, If, Serial>,
	);
	fn node_deleted(node: &super::types::Node<Loc, If, Serial>);
}

pub trait PublicIpModifier {
	fn ip_removed(ip: &PublicIP);
}

pub trait FindNextAuthorTrait<T> {
	fn is_next_block_author(signer: &Signer<T, AuraId>) -> Result<(), Error> {}
}
