// Copyright 2018 Parity Technologies (UK) Ltd.
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

//! Data definitions for the basic_add parachain.

use parachain::codec::{Decode, Encode, Input, Output};

/// Head data for this parachain.
#[derive(Default, Clone)]
pub struct HeadData {
	/// Block number
	pub number: u64,
	/// parent block keccak256
	pub parent_hash: [u8; 32],
	/// hash of post-execution state.
	pub post_state: [u8; 32],
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

/// Block data for this parachain.
#[derive(Default, Clone)]
pub struct BlockData {
	/// State to begin from.
	pub state: u64,
	/// Amount to add (overflowing)
	pub add: u64,
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

pub fn hash_state(state: u64) -> [u8; 32] {
	::tiny_keccak::keccak256(state.encode().as_slice())
}

/// Start state
pub struct StateMismatch;

/// Execute a block body on top of given parent head, producing new parent head
/// if valid.
pub fn execute(parent_hash: [u8; 32], parent_head: HeadData, block_data: &BlockData) -> Result<HeadData, StateMismatch> {
	debug_assert_eq!(parent_hash, ::tiny_keccak::keccak256(&parent_head.encode()));

	if hash_state(block_data.state) != parent_head.post_state {
		return Err(StateMismatch);
	}

	let new_state = block_data.state.saturating_add(block_data.add);

	Ok(HeadData {
		number: parent_head.number + 1,
		parent_hash,
		post_state: hash_state(new_state),
	})
}
