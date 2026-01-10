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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_id() {
        assert_eq!(to_id("Pikachu"), "pikachu");
        assert_eq!(to_id("Mr. Mime"), "mrmime");
        assert_eq!(to_id("Basculin-Blue-Striped"), "basculinbluestriped");
        assert_eq!(to_id("Nidoran♀"), "nidoran");
        assert_eq!(to_id("UPPER CASE"), "uppercase");
    }

    #[test]
    fn test_id_creation() {
        let id = ID::new("Pikachu");
        assert_eq!(id.as_str(), "pikachu");
        assert!(!id.is_empty());

        let empty = ID::empty();
        assert!(empty.is_empty());
    }

    #[test]
    fn test_stats_table() {
        let mut stats = StatsTable::new(100, 80, 70, 90, 75, 95);
        assert_eq!(stats.get(StatID::HP), 100);
        assert_eq!(stats.get(StatID::Spe), 95);

        stats.set(StatID::Atk, 85);
        assert_eq!(stats.get(StatID::Atk), 85);
    }

    #[test]
    fn test_boosts_table() {
        let mut boosts = BoostsTable::new();
        assert_eq!(boosts.get(BoostID::Atk), 0);

        let change = boosts.boost(BoostID::Atk, 2);
        assert_eq!(change, 2);
        assert_eq!(boosts.get(BoostID::Atk), 2);

        // Test clamping at +6
        let change = boosts.boost(BoostID::Atk, 10);
        assert_eq!(change, 4); // Only went up by 4 (from 2 to 6)
        assert_eq!(boosts.get(BoostID::Atk), 6);

        // Test clamping at -6
        boosts.set(BoostID::Def, -6);
        let change = boosts.boost(BoostID::Def, -2);
        assert_eq!(change, 0); // No change, already at min
    }

    #[test]
    fn test_nature() {
        let adamant = Nature::new("Adamant", Some(StatID::Atk), Some(StatID::SpA));
        assert_eq!(adamant.stat_modifier(StatID::Atk), 1.1);
        assert_eq!(adamant.stat_modifier(StatID::SpA), 0.9);
        assert_eq!(adamant.stat_modifier(StatID::Spe), 1.0);
        assert_eq!(adamant.stat_modifier(StatID::HP), 1.0);
    }

    #[test]
    fn test_dex_natures() {
        let mut natures = DexNatures::new(9);

        let adamant = natures.get("Adamant");
        assert_eq!(adamant.name, "Adamant");
        assert_eq!(adamant.plus, Some(StatID::Atk));
        assert_eq!(adamant.minus, Some(StatID::SpA));

        let all = natures.all();
        assert_eq!(all.len(), 25); // 25 natures total
    }

    #[test]
    fn test_dex_types() {
        let mut types = DexTypes::new(9);

        let fire = types.get("Fire");
        assert_eq!(fire.name, "Fire");
        assert!(fire.exists);
        assert_eq!(fire.damage_multiplier("water"), 2.0);
        assert_eq!(fire.damage_multiplier("grass"), 0.5);

        let all = types.all();
        assert_eq!(all.len(), 18); // 18 types in gen 9

        assert!(types.is_name("Fire"));
        assert!(!types.is_name("fire")); // Must be capitalized
        assert!(!types.is_name("NotAType"));
    }

    #[test]
    fn test_dex_types_gen1() {
        let mut types = DexTypes::new(1);

        // Dark and Steel don't exist in Gen 1
        let all = types.all();
        assert_eq!(all.len(), 15); // 15 types in gen 1 (no Dark, Steel, Fairy)
    }

    #[test]
    fn test_dex_stats() {
        let stats = DexStats::new(9);

        assert_eq!(stats.get_id("Attack"), Some(StatID::Atk));
        assert_eq!(stats.get_id("Spd"), Some(StatID::Spe)); // Special case
        assert_eq!(stats.get_id("Speed"), Some(StatID::Spe));

        let ids = stats.ids();
        assert_eq!(ids.len(), 6);

        let short = stats.short_names();
        assert_eq!(short.get(&StatID::SpA), Some(&"SpA"));
    }

    #[test]
    fn test_dex_stats_gen1() {
        let stats = DexStats::new(1);

        let short = stats.short_names();
        assert_eq!(short.get(&StatID::SpA), Some(&"Spc")); // Gen 1 uses "Special"
    }

    #[test]
    fn test_display_traits() {
        let effect = BasicEffect::new("Leftovers");
        assert_eq!(format!("{}", effect), "Leftovers");

        let nature = Nature::new("Adamant", Some(StatID::Atk), Some(StatID::SpA));
        assert_eq!(format!("{}", nature), "Adamant");

        let type_info = TypeInfo::new("Fire");
        assert_eq!(format!("{}", type_info), "Fire");
    }
}
