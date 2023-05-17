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

//! Autogenerated weights for module_transaction_pause
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

/// Weight functions for module_transaction_pause.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_pause::WeightInfo for WeightInfo<T> {
	// Storage: TransactionPause PausedTransactions (r:1 w:1)
	// Proof Skipped: TransactionPause PausedTransactions (max_values: None, max_size: None, mode: Measured)
	fn pause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1140`
		//  Estimated: `4605`
		// Minimum execution time: 20_637 nanoseconds.
		Weight::from_parts(21_156_000, 4605)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:1)
	// Proof Skipped: TransactionPause PausedTransactions (max_values: None, max_size: None, mode: Measured)
	fn unpause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1191`
		//  Estimated: `4656`
		// Minimum execution time: 22_079 nanoseconds.
		Weight::from_parts(22_754_000, 4656)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: TransactionPause PausedEvmPrecompiles (r:1 w:1)
	// Proof Skipped: TransactionPause PausedEvmPrecompiles (max_values: None, max_size: None, mode: Measured)
	fn pause_evm_precompile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1140`
		//  Estimated: `4605`
		// Minimum execution time: 20_398 nanoseconds.
		Weight::from_parts(20_900_000, 4605)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: TransactionPause PausedEvmPrecompiles (r:1 w:1)
	// Proof Skipped: TransactionPause PausedEvmPrecompiles (max_values: None, max_size: None, mode: Measured)
	fn unpause_evm_precompile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1201`
		//  Estimated: `4666`
		// Minimum execution time: 21_350 nanoseconds.
		Weight::from_parts(22_405_000, 4666)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
