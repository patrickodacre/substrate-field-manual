//! Autogenerated weights for pallet_currencydemo
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Alexs-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../../target/release/node-currencydemo
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_currencydemo
// --extrinsic
// *
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output
// pallets/currencydemo/src/weights.rs
// --currencydemo
// ../../.maintain/frame-weight-currencydemo.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_currencydemo.
pub trait WeightInfo {
	fn transfer() -> Weight;
	fn transfer_from() -> Weight;
	fn approve() -> Weight;
}

/// Weights for pallet_currencydemo using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn transfer() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn transfer_from() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn approve() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn transfer() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn transfer_from() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn approve() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}