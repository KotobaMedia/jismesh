pub(crate) mod utils;
pub use utils::{JismeshError, MeshLevel, to_meshcode, to_meshlevel, to_meshpoint};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
