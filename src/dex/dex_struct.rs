//! The main Dex structure

use std::collections::HashMap;

use crate::dex_data::ID;

use super::{
    AbilityData, ConditionData, FormatData, ItemData, MoveData, NatureData, RulesetData,
    SpeciesData, TypeData,
};

/// The main Dex structure
/// JavaScript equivalent: ModdedDex (sim/dex.ts)
/// ~15 fields in JavaScript (data maps, gen, formats, etc.)
#[derive(Debug, Clone, Default)]
pub struct Dex {
    pub species: HashMap<ID, SpeciesData>,
    pub moves: HashMap<ID, MoveData>,
    pub abilities: HashMap<ID, AbilityData>,
    pub items: HashMap<ID, ItemData>,
    pub conditions: HashMap<ID, ConditionData>,
    pub types: HashMap<String, TypeData>,
    pub natures: HashMap<ID, NatureData>,
    pub rulesets: HashMap<ID, RulesetData>,
    /// Aliases map from alias ID to canonical name
    pub aliases: HashMap<ID, String>,
    /// Compound word names with extra hyphens to mark word boundaries
    pub compound_word_names: Vec<String>,
    /// Battle formats
    pub formats: Vec<FormatData>,
    pub gen: u8,
    /// Current mod (e.g., "gen1stadium")
    /// JavaScript: dex.ts - currentMod: string
    pub current_mod: Option<String>,
}

/// Struct to hold JSON data for loading the Dex
/// TODO: Not in JavaScript - Rust-specific helper for JSON deserialization
/// JavaScript loads data directly from require() statements
pub struct DexJsonData<'a> {
    pub species_json: &'a str,
    pub moves_json: &'a str,
    pub abilities_json: &'a str,
    pub items_json: &'a str,
    pub conditions_json: &'a str,
    pub types_json: &'a str,
    pub natures_json: &'a str,
    pub rulesets_json: &'a str,
    pub aliases_json: &'a str,
    pub compound_word_names_json: &'a str,
    pub formats_json: &'a str,
    pub formats_data_json: &'a str,
}

impl Dex {
}
