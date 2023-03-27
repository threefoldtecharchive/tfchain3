pub mod runtimes;
use crate::runtimes::mainnet;
use crate::runtimes::types::*;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::SecretStringError, ed25519, sr25519, Pair};
use std::str::FromStr;
use subxt::config::extrinsic_params::BaseExtrinsicParams;
use subxt::config::polkadot::PlainTip;
use subxt::config::WithExtrinsicParams;
use subxt::SubstrateConfig;
use subxt::{
	tx::{PairSigner, Signer},
	Error, OnlineClient, PolkadotConfig,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Runtime {
	// Local,
	// Devnet,
	// Testnet,
	Mainnet,
}

impl FromStr for Runtime {
	type Err = &'static str;

	fn from_str(v: &str) -> Result<Self, Self::Err> {
		match v {
			// "local" => Ok(Self::Local),
			// "devnet" => Ok(Self::Devnet),
			"mainnet" => Ok(Self::Mainnet),
			// "testnet" => Ok(Self::Testnet),
			_ => Err("unknown runtime"),
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyType {
	Sr25519,
	Ed25519,
}

impl FromStr for KeyType {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"sr25519" => Ok(Self::Sr25519),
			"ed25519" => Ok(Self::Ed25519),
			_ => Err("unknown key type"),
		}
	}
}

#[derive(Clone)]
pub enum KeyPair {
	Sr25519(sr25519::Pair),
	Ed25519(ed25519::Pair),
}

impl KeyPair {
	// create a key pair from a seed prefixed with `0x`. or a BIP-39 phrase
	pub fn from_phrase<S: AsRef<str>>(
		k: KeyType,
		phrase: S,
		password: Option<&str>,
	) -> Result<Self, SecretStringError> {
		let phrase = phrase.as_ref();

		let pair = match k {
			KeyType::Sr25519 => {
				let pair: sr25519::Pair = Pair::from_string(phrase, password)?;
				Self::Sr25519(pair)
			},
			KeyType::Ed25519 => {
				let pair: ed25519::Pair = Pair::from_string(phrase, password)?;
				Self::Ed25519(pair)
			},
		};

		Ok(pair)
	}

	pub fn signer(&self) -> KeypairSigner {
		match self {
			Self::Ed25519(pair) => KeypairSigner(Box::new(PairSigner::new(pair.clone()))),
			Self::Sr25519(pair) => KeypairSigner(Box::new(PairSigner::new(pair.clone()))),
		}
	}
}

pub struct KeypairSigner(Box<dyn Signer<PolkadotConfig> + Send + Sync>);

impl
	subxt::tx::Signer<
		WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>,
	> for KeypairSigner
{
	fn account_id(&self) -> &<WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>> as subxt::Config>::AccountId{
		self.0.account_id()
	}

	fn address(&self) -> <WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>> as subxt::Config>::Address{
		self.0.address()
	}

	fn sign(&self, signer_payload: &[u8]) -> <WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>> as subxt::Config>::Signature{
		self.0.sign(signer_payload)
	}
}

impl From<sr25519::Pair> for KeyPair {
	fn from(value: sr25519::Pair) -> Self {
		Self::Sr25519(value)
	}
}

impl From<ed25519::Pair> for KeyPair {
	fn from(value: ed25519::Pair) -> Self {
		Self::Ed25519(value)
	}
}

#[derive(Clone)]
pub struct Client {
	pub runtime: Runtime,
	pub api: OnlineClient<PolkadotConfig>,
}

macro_rules! call {
    ($self:ident, $name:ident, $($arg:expr),+) => (
        match $self.runtime {
            // Runtime::Local => local::$name($self, $($arg),+).await,
            // Runtime::Devnet => devnet::$name($self, $($arg),+).await,
            // Runtime::Testnet => testnet::$name($self, $($arg),+).await,
            Runtime::Mainnet => mainnet::$name($self, $($arg),+).await,
        }
    )
}

impl Client {
	pub async fn new<U: AsRef<str>>(url: U, runtime: Runtime) -> Result<Client, Error> {
		let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

		Ok(Client { api, runtime })
	}

	pub async fn get_balance(
		&self,
		account: &AccountId32,
		at_block: Option<Hash>,
	) -> Result<Option<SystemAccountInfo>, Error> {
		call!(self, get_balance, account, at_block)
	}

	pub async fn get_block_hash(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<Option<Hash>, Error> {
		call!(self, get_block_hash, block_number)
	}

	pub async fn transfer_native(
		&self,
		keypair: &KeyPair,
		to: AccountId32,
		amount: u128,
	) -> Result<Hash, Error> {
		call!(self, transfer_native, keypair, to, amount)
	}
}
