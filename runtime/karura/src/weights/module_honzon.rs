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

//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-35-209`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_honzon.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon::WeightInfo for WeightInfo<T> {
	// Storage: Honzon Authorization (r:1 w:1)
	// Proof: Honzon Authorization (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn authorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1414`
		//  Estimated: `7245`
		// Minimum execution time: 37_086 nanoseconds.
		Weight::from_parts(37_811_000, 7245)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Honzon Authorization (r:1 w:1)
	// Proof: Honzon Authorization (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn unauthorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1634`
		//  Estimated: `7245`
		// Minimum execution time: 42_064 nanoseconds.
		Weight::from_parts(43_168_000, 7245)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Honzon Authorization (r:4 w:4)
	// Proof: Honzon Authorization (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 4]`.
	fn unauthorize_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1253 + c * (176 ±0)`
		//  Estimated: `4623 + c * (2622 ±0)`
		// Minimum execution time: 22_511 nanoseconds.
		Weight::from_parts(28_321_806, 4623)
			// Standard Error: 181_754
			.saturating_add(Weight::from_parts(6_406_729, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2622).saturating_mul(c.into()))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	fn adjust_loan() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2400`
		//  Estimated: `61419`
		// Minimum execution time: 131_847 nanoseconds.
		Weight::from_parts(136_545_000, 61419)
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Honzon Authorization (r:1 w:0)
	// Proof: Honzon Authorization (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:2 w:2)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:2 w:2)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	fn transfer_loan_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2745`
		//  Estimated: `58751`
		// Minimum execution time: 109_398 nanoseconds.
		Weight::from_parts(112_240_000, 58751)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:6 w:6)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Proof: CdpTreasury DebitPool (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Proof: AuctionManager TotalCollateralInAuction (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:2 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Proof Skipped: AggregatedDex AggregatedSwapPaths (max_values: None, max_size: None, mode: Measured)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	fn close_loan_has_debit_by_dex() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5207`
		//  Estimated: `149158`
		// Minimum execution time: 353_554 nanoseconds.
		Weight::from_parts(358_684_000, 149158)
			.saturating_add(T::DbWeight::get().reads(37))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:2 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Proof Skipped: AggregatedDex AggregatedSwapPaths (max_values: None, max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	fn expand_position_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4708`
		//  Estimated: `108696`
		// Minimum execution time: 261_307 nanoseconds.
		Weight::from_parts(265_242_000, 108696)
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:1 w:1)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Proof: Dex TradingPairStatuses (max_values: None, max_size: Some(195), added: 2670, mode: MaxEncodedLen)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Proof: Dex LiquidityPool (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:2 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Proof Skipped: AggregatedDex AggregatedSwapPaths (max_values: None, max_size: None, mode: Measured)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Proof Skipped: Rewards SharesAndWithdrawnRewards (max_values: None, max_size: None, mode: Measured)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Proof Skipped: Rewards PoolInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	fn shrink_position_debit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4432`
		//  Estimated: `110245`
		// Minimum execution time: 284_192 nanoseconds.
		Weight::from_parts(287_393_000, 110245)
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: CdpEngine CollateralParams (r:2 w:0)
	// Proof: CdpEngine CollateralParams (max_values: None, max_size: Some(135), added: 2610, mode: MaxEncodedLen)
	// Storage: Loans Positions (r:2 w:2)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Loans TotalPositions (r:2 w:2)
	// Proof: Loans TotalPositions (max_values: None, max_size: Some(83), added: 2558, mode: MaxEncodedLen)
	// Storage: CdpEngine DebitExchangeRate (r:2 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:3 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_debit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2887`
		//  Estimated: `68361`
		// Minimum execution time: 160_990 nanoseconds.
		Weight::from_parts(166_485_000, 68361)
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Loans Positions (r:1 w:0)
	// Proof: Loans Positions (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Proof Skipped: Homa TotalStakingBonded (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Proof Skipped: Homa ToBondPool (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Proof Skipped: Homa TotalVoidLiquid (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Proof: CdpEngine DebitExchangeRate (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	fn precompile_get_current_collateral_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1987`
		//  Estimated: `38609`
		// Minimum execution time: 44_115 nanoseconds.
		Weight::from_parts(45_601_000, 38609)
			.saturating_add(T::DbWeight::get().reads(11))
	}
}
