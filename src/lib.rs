//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod abilities;
pub mod battle;
pub mod battle_actions;
pub mod battle_queue;
pub mod battle_stream;
pub mod choice;
pub mod data;
pub mod dex;
pub mod dex_abilities;
pub mod dex_conditions;
pub mod dex_data;
pub mod dex_formats;
pub mod dex_items;
pub mod dex_moves;
pub mod dex_natures;
pub mod dex_species;
pub mod dex_types;
pub mod event;
pub mod event_system;
pub mod field;
pub mod items;
pub mod pokemon;
pub mod prng;
pub mod side;
pub mod state;
pub mod trace;
pub mod team_generator;
pub mod teams;

// Re-export commonly used types for convenience
pub use battle::{Arg, Battle, BattleOptions, PlayerOptions, EffectContext};
pub use battle_actions::{BattleActions, DamageResult};
pub use dex::Dex;
pub use dex_data::{GameType, Gender, SideID, ID};
pub use pokemon::{Pokemon, PokemonSet, TrappedState};
pub use prng::{PRNGSeed, PRNG};
