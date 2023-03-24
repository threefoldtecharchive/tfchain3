use super::types::*;
use crate::Client;
use subxt::{subxt, Error};

#[subxt(runtime_metadata_path = "artifacts/mainnet.scale")]
pub mod mainnet {}
pub use mainnet::runtime_types::frame_system::AccountInfo;

pub type AccountData = mainnet::runtime_types::pallet_balances::AccountData<u128>;
pub type SystemAccountInfo = AccountInfo<u32, AccountData>;

use super::types::SystemAccountInfo as GenericAccountInfo;

pub async fn get_block_hash(
	cl: &Client,
	block_number: Option<BlockNumber>,
) -> Result<Option<Hash>, Error> {
	cl.api.rpc().block_hash(block_number).await
}

pub async fn get_balance(
	cl: &Client,
	account: &AccountId32,
	at_block: Option<Hash>,
) -> Result<Option<GenericAccountInfo>, Error> {
	Ok(cl
		.api
		.storage()
		.at(at_block)
		.await?
		.fetch(&mainnet::storage().system().account(account))
		.await?
		.map(|t| GenericAccountInfo::from(t)))
}
