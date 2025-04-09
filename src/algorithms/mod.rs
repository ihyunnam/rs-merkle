//! This module contains built-in implementations of the [`Hasher`]
//!
//! [`Hasher`]: crate::Hasher
// mod sha256;
// mod sha384;
mod poseidon;
#[cfg(feature = "keccak256")]
mod keccak256;

// pub use sha256::Sha256Algorithm as Sha256;
// pub use sha384::Sha384Algorithm as Sha384;
pub use poseidon::PoseidonAlgorithm as Poseidon;

#[cfg(feature = "keccak256")]
pub use keccak256::Keccak256Algorithm as Keccak256;
