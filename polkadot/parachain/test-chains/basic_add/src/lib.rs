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

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
	not(feature = "std"),
	feature(
		alloc, core_intrinsics, lang_items, panic_implementation, core_panic_info,
		alloc_error_handler
	)
)]

#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate polkadot_parachain as parachain;
extern crate wee_alloc;
extern crate tiny_keccak;
extern crate pwasm_libc;

use parachain::codec::{Decode, Encode, Input, Output};

#[cfg(not(feature = "std"))]
mod wasm;

#[cfg(not(feature = "std"))]
pub use wasm::*;

// Define global allocator.
#[cfg(not(feature = "std"))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Head data for this parachain.
#[derive(Default, Clone)]
struct HeadData {
	// Block number
	number: u64,
	// parent block keccak256
	parent_hash: [u8; 32],
	// hash of post-execution state.
	post_state: [u8; 32],
}

impl Encode for HeadData {
	fn encode_to<T: Output>(&self, dest: &mut T) {
		dest.push(&self.number);
		dest.push(&self.parent_hash);
		dest.push(&self.post_state);
	}
}

impl Decode for HeadData {
	fn decode<I: Input>(input: &mut I) -> Option<Self> {
		Some(HeadData {
			number: Decode::decode(input)?,
			parent_hash: Decode::decode(input)?,
			post_state: Decode::decode(input)?,
		})
	}
}

// Block data for this parachain.
#[derive(Default, Clone)]
struct BlockData {
	// State to begin from.
	state: u64,
	// Amount to add (overflowing)
	add: u64,
}

impl Encode for BlockData {
	fn encode_to<T: Output>(&self, dest: &mut T) {
		dest.push(&self.state);
		dest.push(&self.add);
	}
}

impl Decode for BlockData {
	fn decode<I: Input>(input: &mut I) -> Option<Self> {
		Some(BlockData {
			state: Decode::decode(input)?,
			add: Decode::decode(input)?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use parachain::ValidationParams;

	const TEST_CODE: &[u8] = include_bytes!("../wasm/test.wasm");

	fn hash_state(state: u64) -> [u8; 32] {
		::tiny_keccak::keccak256(state.encode().as_slice())
	}

	fn hash_head(head: &HeadData) -> [u8; 32] {
		::tiny_keccak::keccak256(head.encode().as_slice())
	}

	#[test]
	fn execute_good_on_parent() {
		let parent_head = HeadData {
			number: 0,
			parent_hash: [0; 32],
			post_state: hash_state(0),
		};

		let block_data = BlockData {
			state: 0,
			add: 512,
		};

		let ret = parachain::wasm::validate_candidate(TEST_CODE, ValidationParams {
			parent_head: parent_head.encode(),
			block_data: block_data.encode(),
		}).unwrap();

		let new_head = HeadData::decode(&mut &ret.head_data[..]).unwrap();

		assert_eq!(new_head.number, 1);
		assert_eq!(new_head.parent_hash, hash_head(&parent_head));
		assert_eq!(new_head.post_state, hash_state(512));
	}
}
