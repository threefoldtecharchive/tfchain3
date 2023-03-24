use super::types::*;
use crate::{Client, KeyPair};
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

pub async fn transfer_native(
	cl: &Client,
	kp: &KeyPair,
	dest: AccountId32,
	value: u128,
) -> Result<Hash, Error> {
	let transfer_tx = mainnet::tx().balances().transfer(dest.into(), value);

	let s = &kp.signer();

	let transfer = cl
		.api
		.tx()
		.sign_and_submit_then_watch_default(&transfer_tx, s)
		.await?
		.wait_for_finalized_success()
		.await?;

	let balances_transfer = transfer.find_first::<mainnet::balances::events::Transfer>()?;

	if let Some(_) = balances_transfer {
		Ok(transfer.block_hash())
	} else {
		Err(Error::Other(String::from("failed to transfer")))
	}
}
