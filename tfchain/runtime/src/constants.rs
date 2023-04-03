// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
pub type BlockNumber = u32;

pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// Time and blocks.
pub mod time {
	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_aura` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 6000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
	pub const SECS_PER_HOUR: u64 = 3600;
	pub const EPOCH_DURATION_IN_BLOCKS: super::BlockNumber = 1 * HOURS;
	pub const EPOCH_DURATION_IN_SLOTS: super::BlockNumber = prod_or_fast!(4 * HOURS, 1 * MINUTES);

	// These time units are defined in number of blocks.
	pub const MINUTES: super::BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as super::BlockNumber);
	pub const HOURS: super::BlockNumber = MINUTES * 60;
	pub const DAYS: super::BlockNumber = HOURS * 24;
}

/// Money matters.
pub mod currency {
	use crate::Balance;

	pub const CHIS: Balance = 1_000_000_0;
	pub const UNITS: Balance = CHIS;
	pub const CENTS: Balance = UNITS / 100;
	pub const MILLICENTS: Balance = CENTS / 1_000;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 1 * UNITS + (bytes as Balance) * 5 * MILLICENTS
	}
}

/// Fee-related.
pub mod fee {
	use crate::Balance;
	use frame_support::weights::constants::ExtrinsicBaseWeight;
	use frame_support::weights::{
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use smallvec::smallvec;
	pub use sp_runtime::{PerThing, Perbill};

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, system::MaximumBlockWeight]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFeeStruct;
	impl WeightToFeePolynomial for WeightToFeeStruct {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Tfchain, extrinsic base weight (smallest non-zero weight) is mapped to 1 CENT:
			let p = super::currency::CENTS;
			let q = 1 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: PerThing::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}

#[cfg(test)]
mod tests {
	use super::currency::{CENTS, MILLICENTS};
	use super::fee::WeightToFeeStruct;
	use frame_support::weights::constants::ExtrinsicBaseWeight;
	use frame_support::weights::WeightToFee;

	#[test]
	// This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
	fn extrinsic_base_fee_is_correct() {
		// `ExtrinsicBaseWeight` should cost one CENT
		log::info!("Base: {}", ExtrinsicBaseWeight::get());
		let x = WeightToFeeStruct::weight_to_fee(&ExtrinsicBaseWeight::get());
		let y = CENTS / 1;
		assert!(x.max(y) - x.min(y) < MILLICENTS);
	}
}
