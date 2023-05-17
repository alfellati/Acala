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

//! Autogenerated weights for module_auction_manager
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

/// Weight functions for module_auction_manager.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_auction_manager::WeightInfo for WeightInfo<T> {
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Proof: EmergencyShutdown IsShutdown (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: AuctionManager CollateralAuctions (r:1 w:1)
	// Proof: AuctionManager CollateralAuctions (max_values: None, max_size: Some(139), added: 2614, mode: MaxEncodedLen)
	// Storage: Auction Auctions (r:1 w:1)
	// Proof: Auction Auctions (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Proof: Prices LockedPrice (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry AssetMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Proof: AcalaOracle Values (max_values: None, max_size: Some(75), added: 2550, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Proof: AuctionManager TotalCollateralInAuction (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Proof: AuctionManager TotalTargetInAuction (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Auction AuctionEndTime (r:0 w:1)
	// Proof: Auction AuctionEndTime (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	fn cancel_collateral_auction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2795`
		//  Estimated: `44437`
		// Minimum execution time: 106_741 nanoseconds.
		Weight::from_parts(109_443_000, 44437)
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
