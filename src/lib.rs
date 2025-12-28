//! Pokemon Showdown Battle Simulator - Rust Port
//!
//! This is a high-performance Rust implementation of the Pokemon Showdown
//! battle simulator, designed to be compatible with the original TypeScript
//! implementation.

pub mod prng;
pub mod dex_data;
pub mod dex;
pub mod field;
pub mod battle_queue;
pub mod pokemon;
pub mod side;
pub mod battle;
pub mod battle_actions;
pub mod abilities;
pub mod items;
pub mod event;
pub mod event_system;
pub mod data;
pub mod state;
pub mod team_validator;
pub mod battle_stream;
pub mod choice;
pub mod random_teams;
pub mod move_types;
pub mod teams;
