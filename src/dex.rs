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
