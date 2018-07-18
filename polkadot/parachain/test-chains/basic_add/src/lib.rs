// Copyright 2017 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Basic parachain that adds a number as part of its state.

#![cfg_attr(feature = "wasm", no_std)]
#![cfg_attr(
	feature = "wasm",
	feature(
		alloc, core_intrinsics, lang_items, panic_implementation, core_panic_info,
		alloc_error_handler
	)
)]

#[cfg(feature = "wasm")]
extern crate alloc;

#[cfg(feature = "wasm")]
extern crate wee_alloc;

#[cfg(feature = "wasm")]
extern crate pwasm_libc;

extern crate polkadot_parachain as parachain;
extern crate tiny_keccak;

mod common;
pub use common::{execute, HeadData, BlockData};

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::*;

// Define global allocator.
#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
