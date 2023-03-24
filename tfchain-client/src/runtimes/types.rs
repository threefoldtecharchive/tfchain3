use subxt::{Config, PolkadotConfig};

pub use frame_system::AccountInfo;
pub use pallet_balances::AccountData;
pub type Hash = <PolkadotConfig as Config>::Hash;
pub type BlockNumber = subxt::rpc::types::BlockNumber;
pub use subxt::utils::AccountId32;

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

use super::mainnet::SystemAccountInfo as MainnetSystemAccountInfo;

impl From<MainnetSystemAccountInfo> for SystemAccountInfo {
	fn from(info: MainnetSystemAccountInfo) -> Self {
		SystemAccountInfo {
			nonce: info.nonce,
			consumers: info.consumers,
			providers: info.providers,
			sufficients: info.sufficients,
			data: pallet_balances::AccountData {
				free: info.data.free,
				fee_frozen: info.data.fee_frozen,
				misc_frozen: info.data.misc_frozen,
				reserved: info.data.reserved,
			},
		}
	}
}
