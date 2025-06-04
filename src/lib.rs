/// メッシュコード一覧
pub mod codes;
pub(crate) mod utils;
pub use utils::{
    JismeshError, MeshCode, MeshLevel, to_envelope, to_intersects, to_meshcode, to_meshlevel,
    to_meshpoint,
};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
