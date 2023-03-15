use pallet_staking::{Forcing, StakerStatus};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use tfchain_runtime::{
	currency::CHIS, AccountId, AssetsConfig, BabeConfig, BabeId, BalancesConfig, CouncilConfig,
	CouncilMembershipConfig, GenesisConfig, GrandpaConfig, GridContractsConfig, GridStoreConfig,
	Perbill, SessionConfig, SessionKeys, Signature, StakingConfig, SudoConfig, SystemConfig,
	TftPriceConfig, TreasuryConfig, WASM_BINARY,
};

const DEV_PROTOCOL_ID: &str = "chi";

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let properties = Some(
		serde_json::json!({
			"tokenDecimals": 7,
			"tokenSymbol": "CHI",
		})
		.as_object()
		.expect("Map given; qed")
		.clone(),
	);

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_from_seed::<BabeId>("Alice"),
					get_from_seed::<GrandpaId>("Alice"),
				)], // Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// TFT price pallet min price
				10,
				// TFT price pallet max price
				1000,
				// billing frequency
				5,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		Some(DEV_PROTOCOL_ID),
		// Properties
		properties,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let properties = Some(
		serde_json::json!({
			"tokenDecimals": 7,
			"tokenSymbol": "CHI",
		})
		.as_object()
		.expect("Map given; qed")
		.clone(),
	);

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Chi Testnet",
		// ID
		"local_chi_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_from_seed::<BabeId>("Alice"),
						get_from_seed::<GrandpaId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						get_from_seed::<BabeId>("Bob"),
						get_from_seed::<GrandpaId>("Bob"),
					),
				], // Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// TFT price pallet min price
				10,
				// TFT price pallet max price
				1000,
				// billing frequency
				5,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(DEV_PROTOCOL_ID),
		// Properties
		None,
		properties,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	council_members: Vec<AccountId>,
	foundation_account: AccountId,
	sales_account: AccountId,
	min_tft_price: u32,
	max_tft_price: u32,
	billing_frequency: u64,
) -> GenesisConfig {
	const ENDOWMENT: u128 = 1_000_000_000_000 * CHIS;
	const STASH: u128 = 1_000_000_000 * CHIS;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		grandpa: GrandpaConfig { authorities: Default::default() },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						SessionKeys { babe: x.2.clone(), grandpa: x.3.clone() },
					)
				})
				.collect::<Vec<_>>(),
		},
		transaction_payment: Default::default(),
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(tfchain_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		council: CouncilConfig::default(),
		council_membership: CouncilMembershipConfig {
			members: council_members.try_into().unwrap(),
			phantom: Default::default(),
		},
		assets: AssetsConfig { ..Default::default() },
		treasury: TreasuryConfig {},
		tft_price: TftPriceConfig { min_tft_price, max_tft_price, _data: std::marker::PhantomData },
		grid_contracts: GridContractsConfig { billing_frequency },
		grid_store: GridStoreConfig {
			su_price_value: 50000,
			su_price_unit: 4,
			nu_price_value: 15000,
			nu_price_unit: 4,
			cu_price_value: 100000,
			cu_price_unit: 4,
			ipu_price_value: 40000,
			ipu_price_unit: 4,
			unique_name_price_value: 2500,
			domain_name_price_value: 5000,
			foundation_account: Some(foundation_account),
			sales_account: Some(sales_account),
			farming_policy_diy_cu: 2400,
			farming_policy_diy_su: 1000,
			farming_policy_diy_nu: 30,
			farming_policy_diy_ipu: 5,
			farming_policy_diy_minimal_uptime: 95,
			farming_policy_certified_cu: 3000,
			farming_policy_certified_su: 1250,
			farming_policy_certified_nu: 38,
			farming_policy_certified_ipu: 6,
			farming_policy_certified_minimal_uptime: 95,
			discount_for_dedication_nodes: 50,
			connection_price: 80,
		},
	}
}
