//! Dex Data - Core types and ID system
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the core type definitions used throughout the simulator.

use unicode_normalization::UnicodeNormalization;

// Type modules
mod id;
mod gender;
mod stats;
mod boosts;
mod enums;
mod basic_effect;
mod nature;
mod type_info;
mod dex_stats;

// Re-export types from submodules
pub use id::ID;
pub use gender::Gender;
pub use stats::{StatID, StatsTable};
pub use boosts::{BoostID, BoostsTable};
pub use enums::{EffectType, GameType, MoveTarget, Nonstandard, SideID};
pub use basic_effect::BasicEffect;
pub use nature::{DexNatures, Nature};
pub use type_info::{capitalize_first, DexTypes, TypeInfo};
pub use dex_stats::DexStats;

/// Converts anything to an ID. An ID must have only lowercase alphanumeric characters.
// TypeScript source:
// /**
// * Converts anything to an ID. An ID must have only lowercase alphanumeric
// * characters.
// *
// * If a string is passed, it will be converted to lowercase and
// * non-alphanumeric characters will be stripped.
// *
// * If an object with an ID is passed, its ID will be returned.
// * Otherwise, an empty string will be returned.
// *
// * Generally assigned to the global toID, because of how
// * commonly it's used.
// */
// export function toID(text: any): ID {
// 	if (typeof text !== 'string') {
// 		if (text) text = text.id || text.userid || text.roomid || text;
// 		if (typeof text === 'number') text = `${text}`;
// 		else if (typeof text !== 'string') return '';
// 	}
// 	return text.toLowerCase().replace(/[^a-z0-9]+/g, '') as ID;
// }
//
pub fn to_id(text: &str) -> String {
    // Normalize to NFD (decomposed form) to match JSON data format
    // This ensures "é" (U+00E9) becomes "e" + "́" (U+0301)
    // Then we filter for ASCII characters, keeping the base letter
    text.nfd()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
