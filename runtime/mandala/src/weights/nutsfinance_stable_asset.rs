// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for nutsfinance_stable_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-43-79`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for nutsfinance_stable_asset.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> nutsfinance_stable_asset::WeightInfo for WeightInfo<T> {
	// Storage: StableAsset PoolCount (r:1 w:1)
	// Proof Skipped: StableAsset PoolCount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1879`
		//  Estimated: `12301`
		// Minimum execution time: 34_287 nanoseconds.
		Weight::from_parts(34_917_000, 12301)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_a() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1506`
		//  Estimated: `4971`
		// Minimum execution time: 26_772 nanoseconds.
		Weight::from_parts(27_910_000, 4971)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_fees() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1506`
		//  Estimated: `4971`
		// Minimum execution time: 25_656 nanoseconds.
		Weight::from_parts(26_763_000, 4971)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	fn modify_recipients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1506`
		//  Estimated: `4971`
		// Minimum execution time: 25_742 nanoseconds.
		Weight::from_parts(26_716_000, 4971)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn mint(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3281 + u * (222 ±0)`
		//  Estimated: `37799 + u * (7250 ±2)`
		// Minimum execution time: 162_357 nanoseconds.
		Weight::from_parts(77_215_243, 37799)
			// Standard Error: 407_378
			.saturating_add(Weight::from_parts(48_370_556, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7250).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:6 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn swap(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3206 + u * (221 ±0)`
		//  Estimated: `37437 + u * (4628 ±0)`
		// Minimum execution time: 1_503_507 nanoseconds.
		Weight::from_parts(177_409_847, 37437)
			// Standard Error: 4_291_354
			.saturating_add(Weight::from_parts(722_027_873, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 4628).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_proportion(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3148 + u * (286 ±0)`
		//  Estimated: `34520 + u * (7575 ±32)`
		// Minimum execution time: 179_787 nanoseconds.
		Weight::from_parts(104_746_434, 34520)
			// Standard Error: 274_234
			.saturating_add(Weight::from_parts(42_109_989, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7575).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_single(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2428 + u * (227 ±0)`
		//  Estimated: `29963 + u * (1536 ±0)`
		// Minimum execution time: 1_007_389 nanoseconds.
		Weight::from_parts(482_408_904, 29963)
			// Standard Error: 1_007_245
			.saturating_add(Weight::from_parts(264_804_569, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 1536).saturating_mul(u.into()))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:10 w:10)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	/// The range of component `u` is `[2, 5]`.
	fn redeem_multi(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3148 + u * (286 ±0)`
		//  Estimated: `34520 + u * (7575 ±0)`
		// Minimum execution time: 157_315 nanoseconds.
		Weight::from_parts(67_302_617, 34520)
			// Standard Error: 449_610
			.saturating_add(Weight::from_parts(50_748_202, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 7575).saturating_mul(u.into()))
	}
}
