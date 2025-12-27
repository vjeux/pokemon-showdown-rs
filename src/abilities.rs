//! Ability System
//!
//! This module provides access to ability data loaded from data/abilities.json.
//! All ability data is loaded dynamically from JSON - no manual implementations.
//!
//! Ability effects are handled through the callback dispatch system in
//! src/data/ability_callbacks/mod.rs

// Re-export ability data type from dex
pub use crate::dex::AbilityData;

// The Dex loads all abilities from data/abilities.json at initialization.
// Access ability data via Battle.dex.get_ability(ability_id)
//
// Example:
//   if let Some(ability_data) = battle.dex.get_ability(&ID::new("intimidate")) {
//       println!("Ability: {} (rating: {})", ability_data.name, ability_data.rating.unwrap_or(0.0));
//   }
