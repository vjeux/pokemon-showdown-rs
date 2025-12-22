//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod prng;
pub mod dex_data;

// Re-export main types
pub use prng::{PRNG, PRNGSeed, Gen5RNG};
pub use dex_data::{ID, to_id, Gender, StatID, StatsTable, BoostID, BoostsTable};
pub use dex_data::{EffectType, Nonstandard, GameType, SideID, MoveTarget};
pub use dex_data::{EffectState, BasicEffect, Nature, TypeInfo};
