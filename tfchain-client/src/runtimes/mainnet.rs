use super::types::*;
use crate::{Client, KeyPair};
use subxt::{subxt, Error};

#[subxt(runtime_metadata_path = "artifacts/mainnet.scale", derive_for_all_types = "Eq, PartialEq")]
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

// Wrapper around the transfer function of the balances pallet
// This function will wait for the transaction to be finalized and return the block hash
// If the transaction fails, it will return an error
// If the transaction is successful, it will return the block hash
pub async fn transfer_native(
	cl: &Client,
	kp: &KeyPair,
	dest: AccountId32,
	amount: u128,
) -> Result<Hash, Error> {
	let transfer_tx = mainnet::tx().balances().transfer(dest.clone().into(), amount);

	let s = &kp.signer();

	let events = cl
		.api
		.tx()
		.sign_and_submit_then_watch_default(&transfer_tx, s)
		.await?
		.wait_for_finalized_success()
		.await?;

	let expected_event = mainnet::balances::events::Transfer {
		from: s.0.account_id().clone(),
		to: dest.into(),
		amount,
	};

	let exists = events
		.find::<mainnet::balances::events::Transfer>()
		.any(|e| e.map(|x| assert_eq!(x, expected_event)).is_ok());

	if exists {
		Ok(events.block_hash())
	} else {
		Err(Error::Other(String::from("failed to transfer")))
	}
}
