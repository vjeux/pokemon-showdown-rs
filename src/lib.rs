//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod prng;
pub mod dex_data;
pub mod field;
pub mod battle_queue;
pub mod pokemon;
pub mod side;
pub mod battle;

// Re-export main types
pub use prng::{PRNG, PRNGSeed, Gen5RNG};
pub use dex_data::{ID, to_id, Gender, StatID, StatsTable, BoostID, BoostsTable};
pub use dex_data::{EffectType, Nonstandard, GameType, SideID, MoveTarget};
pub use dex_data::{EffectState, BasicEffect, Nature, TypeInfo};
pub use field::Field;
pub use battle_queue::{BattleQueue, Action, MoveAction, SwitchAction};
pub use pokemon::{Pokemon, PokemonSet, MoveSlot};
pub use side::Side;
pub use battle::{Battle, BattleOptions, PlayerOptions};
