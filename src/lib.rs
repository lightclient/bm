#![warn(missing_docs)]

//! Binary merkle tree implementation.

mod traits;
mod raw;
mod vec;
mod empty;
mod tuple;
mod packed;
mod utils;

pub use crate::traits::{MerkleDB, InMemoryMerkleDB, Value, ValueOf, IntermediateOf, IntermediateSizeOf, EndOf};
pub use crate::raw::{MerkleIndex, MerkleRaw};
pub use crate::empty::MerkleEmpty;
pub use crate::vec::MerkleVec;
pub use crate::tuple::MerkleTuple;
pub use crate::packed::{MerklePackedTuple, MerklePackedVec};
