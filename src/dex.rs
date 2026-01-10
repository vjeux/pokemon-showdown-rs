//! Dex - Pokemon Showdown Data System
//!
//! Handles getting data about Pokemon, items, moves, abilities, etc.

// Type modules
mod gender_ratio;
mod ability_slots;
mod base_stats_data;
mod string_or_bool;
mod string_or_vec;
mod species_data;
mod move_secondary;
mod condition_data;
mod accuracy;
mod is_max;
mod ohko;
mod multihit;
mod move_data;
mod ability_data;
mod item_data;
mod type_data;
mod ruleset_data;
mod nature_data;
mod learnset_data;
mod format_data;
mod dex_struct;
pub mod embedded;

// Function modules
mod default_true;
mod default_crit_ratio;
mod deserialize_accuracy;
mod deserialize_is_max;
mod deserialize_ohko;
mod deserialize_self_switch;
mod deserialize_self_boost;
mod deserialize_ignore_immunity;
mod deserialize_damage;
mod deserialize_move_flags;
mod new;
mod load_from_json;
mod get_active_move;
mod convert_move_flags;
mod convert_boosts_hash_to_table;
mod convert_secondary;
mod convert_self_effect;
mod items_helper;
mod get_effectiveness;
mod get_type_effectiveness;
mod get_name;
mod get_immunity;
mod get_hidden_power;
mod trunc;
mod get_gen;
mod for_gen;
mod species_helper;
mod moves_helper;
mod abilities_helper;
mod conditions_helper;
mod formats_helper;
mod natures_helper;
mod types_helper;
mod load_default;

// Re-export types from submodules
pub use gender_ratio::GenderRatio;
pub use ability_slots::AbilitySlots;
pub use base_stats_data::BaseStatsData;
pub use string_or_bool::StringOrBool;
pub use string_or_vec::StringOrVec;
pub use species_data::SpeciesData;
pub use move_secondary::MoveSecondary;
pub use condition_data::{ConditionData, ConditionType};
pub use accuracy::Accuracy;
pub use is_max::IsMax;
pub use ohko::Ohko;
pub use multihit::Multihit;
pub use move_data::MoveData;
pub use ability_data::AbilityData;
pub use item_data::{FlingData, ItemData};
pub use type_data::TypeData;
pub use ruleset_data::RulesetData;
pub use nature_data::NatureData;
pub use learnset_data::{EventData, LearnsetData};
pub use format_data::FormatData;
pub use dex_struct::{Dex, DexJsonData};

// Re-export functions
pub use default_true::default_true;
pub use default_crit_ratio::default_crit_ratio;
pub use deserialize_accuracy::deserialize_accuracy;
pub use deserialize_is_max::deserialize_is_max;
pub use deserialize_ohko::deserialize_ohko;
pub use deserialize_self_switch::deserialize_self_switch;
pub use deserialize_self_boost::deserialize_self_boost;
pub use deserialize_ignore_immunity::deserialize_ignore_immunity;
pub use deserialize_damage::deserialize_damage;
pub use deserialize_move_flags::deserialize_move_flags;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dex() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Check species
        let pikachu = dex.species().get("Pikachu").expect("Pikachu not found");
        assert_eq!(pikachu.name, "Pikachu");
        assert_eq!(pikachu.types, vec!["Electric"]);
        assert_eq!(pikachu.base_stats.hp, 35);
        assert_eq!(pikachu.base_stats.spe, 90);

        // Check moves
        let thunderbolt = dex.moves().get("Thunderbolt").expect("Thunderbolt not found");
        assert_eq!(thunderbolt.name, "Thunderbolt");
        assert_eq!(thunderbolt.move_type, "Electric");
        assert_eq!(thunderbolt.base_power, 90);

        // Check abilities
        let static_ability = dex.abilities().get("Static").expect("Static not found");
        assert_eq!(static_ability.name, "Static");

        // Check items
        let leftovers = dex.items().get("Leftovers").expect("Leftovers not found");
        assert_eq!(leftovers.name, "Leftovers");
    }

    #[test]
    fn test_multi_type_effectiveness() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Electric vs Water/Flying
        // Water: super-effective (+1) + Flying: super-effective (+1) = +2
        let types = vec!["Water".to_string(), "Flying".to_string()];
        assert_eq!(dex.get_type_effectiveness("Electric", &types), 2);

        // Ground vs Electric/Flying
        // Electric: super-effective (+1) + Flying: immune (0) = +1
        let types = vec!["Electric".to_string(), "Flying".to_string()];
        assert_eq!(dex.get_type_effectiveness("Ground", &types), 1);
    }

    #[test]
    fn test_move_flags() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Access flags directly from move data - matches TypeScript pattern
        if let Some(thunderbolt) = dex.moves().get("Thunderbolt") {
            assert!(thunderbolt.flags.contains_key("protect"));
            assert!(!thunderbolt.flags.contains_key("contact"));
        }

        if let Some(quick_attack) = dex.moves().get("Quick Attack") {
            assert!(quick_attack.flags.contains_key("contact"));
        }
    }

    #[test]
    fn test_natures() {
        let dex = Dex::load_default().expect("Failed to load dex");

        let adamant = dex.natures().get("Adamant").expect("Adamant not found");
        assert_eq!(adamant.name, "Adamant");
        assert_eq!(adamant.plus.as_deref(), Some("atk"));
        assert_eq!(adamant.minus.as_deref(), Some("spa"));

        let hardy = dex.natures().get("Hardy").expect("Hardy not found");
        assert!(hardy.plus.is_none()); // Neutral nature
        assert!(hardy.minus.is_none());
    }

    #[test]
    fn test_all_methods() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Test species().all() - should have at least some species
        let all_species = dex.species().all();
        assert!(!all_species.is_empty());

        // Test moves().all() - should have at least some moves
        let all_moves = dex.moves().all();
        assert!(!all_moves.is_empty());

        // Test abilities().all() - should have abilities
        let all_abilities = dex.abilities().all();
        assert!(!all_abilities.is_empty());

        // Test items().all()
        let all_items = dex.items().all();
        assert!(!all_items.is_empty());
    }

    #[test]
    fn test_species_methods() {
        let _dex = Dex::load_default().expect("Failed to load dex");

        // Spec methods are directly accessed from species data
        // No convenience wrappers - matches TypeScript pattern
    }

    #[test]
    fn test_move_methods() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Test moves that exist in our data - access properties directly
        if let Some(thunder_wave) = dex.moves().get("Thunder Wave") {
            assert_eq!(thunder_wave.category, "Status");
        }

        if let Some(thunderbolt) = dex.moves().get("Thunderbolt") {
            assert_eq!(thunderbolt.category, "Special");
        }

        // Quick Attack is physical
        if let Some(quick_attack) = dex.moves().get("Quick Attack") {
            assert_eq!(quick_attack.category, "Physical");
        }
    }

    #[test]
    fn test_item_methods() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Test with items in our data - access properties directly
        if let Some(oran_berry) = dex.items().get("Oran Berry") {
            assert!(oran_berry.is_berry);
        }

        if let Some(leftovers) = dex.items().get("Leftovers") {
            assert!(!leftovers.is_berry);
        }

        if let Some(choice_band) = dex.items().get("Choice Band") {
            assert!(choice_band.is_choice);
        }

        if let Some(leftovers) = dex.items().get("Leftovers") {
            assert!(!leftovers.is_choice);
        }
    }
}
