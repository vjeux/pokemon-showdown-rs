//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod prng;

// Re-export main types
pub use prng::{PRNG, PRNGSeed, Gen5RNG};
