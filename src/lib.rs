#![warn(missing_docs)]

//! Binary merkle tree implementation.

mod traits;
mod memory;
mod raw;
mod index;
mod vector;
mod list;
mod packed;
mod proving;
mod length;

pub mod serialize;

pub use crate::traits::{Backend, Value, ValueOf, IntermediateOf, EndOf, Dangling, Owned, RootStatus, Error, Sequence, Tree, Leak};
pub use crate::memory::{InMemoryBackend, InMemoryBackendError, NoopBackend, NoopBackendError};
pub use crate::raw::{Raw, OwnedRaw, DanglingRaw};
pub use crate::index::{Index, IndexSelection, IndexRoute};
pub use crate::vector::{Vector, OwnedVector, DanglingVector};
pub use crate::list::{List, OwnedList, DanglingList};
pub use crate::packed::{PackedVector, OwnedPackedVector, DanglingPackedVector,
                        PackedList, OwnedPackedList, DanglingPackedList};
pub use crate::proving::ProvingBackend;
pub use crate::length::LengthMixed;
