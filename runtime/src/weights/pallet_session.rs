// Copyright 2019-2021 Dhiway.
// This file is part of the CORD Platform.

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0 for pallet_session
//! DATE: 2021-05-09, STEPS: `[3, ]`, REPEAT: 2,
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev")

// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --steps=3
// --repeat=2
// --pallet=pallet_session
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Interpreted
// --heap-pages=4096
// --output=./runtime/src/weights/
// --template=./.maintain/weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_session.
pub trait WeightInfo {
fn set_keys() -> Weight;
fn purge_keys() -> Weight;
}

//// Weights for pallet_session using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
		impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
				fn set_keys() -> Weight {
				(382_000_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(6 as Weight))
				.saturating_add(T::DbWeight::get().writes(5 as Weight))
				}
				fn purge_keys() -> Weight {
				(286_000_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(2 as Weight))
				.saturating_add(T::DbWeight::get().writes(5 as Weight))
				}
				}

				// For backwards compatibility and tests
				impl WeightInfo for () {
				fn set_keys() -> Weight {
				(382_000_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(6 as Weight))
				.saturating_add(RocksDbWeight::get().writes(5 as Weight))
				}
				fn purge_keys() -> Weight {
				(286_000_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(2 as Weight))
				.saturating_add(RocksDbWeight::get().writes(5 as Weight))
				}
				}