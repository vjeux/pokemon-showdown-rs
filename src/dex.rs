//! Dex - Pokemon Showdown Data System
//!
//! Handles getting data about Pokemon, items, moves, abilities, etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dex_data::{StatsTable, ID};

// Function modules
mod default_true;
mod default_crit_ratio;
mod deserialize_accuracy;
mod deserialize_is_max;
mod deserialize_ohko;
mod new;
mod load_from_json;
mod get_active_move;
mod convert_move_flags;
mod convert_boosts_hash_to_table;
mod convert_secondary;
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
mod formats_helper;
mod natures_helper;
mod types_helper;
mod load_default;
pub use default_true::default_true;
pub use default_crit_ratio::default_crit_ratio;
pub use deserialize_accuracy::deserialize_accuracy;
pub use deserialize_is_max::deserialize_is_max;
pub use deserialize_ohko::deserialize_ohko;

/// Gender ratio structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct GenderRatio {
    #[serde(default)]
    pub m: f64,
    #[serde(default)]
    pub f: f64,
}

/// Abilities structure (mapping slot to ability name)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AbilitySlots {
    #[serde(rename = "0", default)]
    pub slot0: Option<String>,
    #[serde(rename = "1", default)]
    pub slot1: Option<String>,
    #[serde(rename = "H", default)]
    pub hidden: Option<String>,
    #[serde(rename = "S", default)]
    pub special: Option<String>,
}

/// Base stats as stored in JSON
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseStatsData {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

impl From<BaseStatsData> for StatsTable {
    fn from(data: BaseStatsData) -> Self {
        StatsTable {
            hp: data.hp,
            atk: data.atk,
            def: data.def,
            spa: data.spa,
            spd: data.spd,
            spe: data.spe,
        }
    }
}

/// Species data from the pokedex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesData {
    #[serde(default)]
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub base_stats: BaseStatsData,
    #[serde(default)]
    pub abilities: AbilitySlots,
    #[serde(default)]
    pub heightm: f64,
    #[serde(default)]
    pub weightkg: f64,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub gender_ratio: Option<GenderRatio>,
    #[serde(default)]
    pub evos: Vec<String>,
    #[serde(default)]
    pub prevo: Option<String>,
    #[serde(default)]
    pub evo_level: Option<i32>,
    #[serde(default)]
    pub base_species: Option<String>,
    #[serde(default)]
    pub forme: Option<String>,
    #[serde(default)]
    pub other_formes: Vec<String>,
    #[serde(default)]
    pub cosmetic_formes: Vec<String>,
    #[serde(default)]
    pub is_cosmetic_forme: bool,
    #[serde(default)]
    pub gen: Option<u8>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "eggGroups", default)]
    pub egg_groups: Vec<String>,
    #[serde(rename = "battleOnly", default)]
    pub battle_only: StringOrVec,
    #[serde(rename = "formeOrder", default)]
    pub forme_order: Vec<String>,
    #[serde(rename = "requiredItems", default)]
    pub required_items: Vec<String>,
    // Format data fields
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub doubles_tier: Option<String>,
    #[serde(rename = "natDexTier", default)]
    pub nat_dex_tier: Option<String>,
    #[serde(default)]
    pub is_nonstandard: Option<String>,
    #[serde(default = "default_true")]
    pub exists: bool,
}

/// Move secondary effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSecondary {
    #[serde(default)]
    pub chance: Option<i32>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub boosts: Option<HashMap<String, i32>>,
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status_secondary: Option<String>,
}

/// Move data from the move list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveData {
    #[serde(default)]
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub id: ID, // Move ID (computed from name or provided)
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    #[serde(default)]
    pub base_power: i32,
    #[serde(default, deserialize_with = "deserialize_accuracy")]
    pub accuracy: Accuracy,
    #[serde(default)]
    pub pp: i32,
    #[serde(default)]
    pub priority: i8,
    #[serde(default)]
    pub target: String,
    #[serde(rename = "critRatio", default = "default_crit_ratio")]
    pub crit_ratio: i32,
    #[serde(default)]
    pub secondary: Option<MoveSecondary>,
    #[serde(default)]
    pub secondaries: Option<Vec<MoveSecondary>>,
    #[serde(default, rename = "self")]
    pub self_effect: Option<MoveSecondary>,
    #[serde(default)]
    pub flags: HashMap<String, i32>,
    #[serde(default)]
    pub boosts: Option<HashMap<String, i32>>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
    #[serde(default)]
    pub drain: Option<(i32, i32)>,
    #[serde(default)]
    pub recoil: Option<(i32, i32)>,
    #[serde(default)]
    pub heal: Option<(i32, i32)>,
    #[serde(default)]
    pub multihit: Option<Multihit>,
    #[serde(rename = "isZ", default)]
    pub is_z: Option<String>,
    #[serde(rename = "isMax", default, deserialize_with = "deserialize_is_max")]
    pub is_max: Option<IsMax>,
    #[serde(default, deserialize_with = "deserialize_ohko")]
    pub ohko: Option<Ohko>,
    #[serde(default)]
    pub selfdestruct: Option<String>,
    #[serde(rename = "tracksTarget", default)]
    pub tracks_target: Option<bool>,
    #[serde(rename = "smartTarget", default)]
    pub smart_target: Option<bool>,
    #[serde(rename = "baseMove", default)]
    pub base_move: Option<ID>,
    #[serde(rename = "isZOrMaxPowered", default)]
    pub is_z_or_max_powered: bool,
}

/// Accuracy can be a number or true (always hits)
#[derive(Debug, Clone)]
pub enum Accuracy {
    Percent(i32),
    AlwaysHits,
}

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy::Percent(100)
    }
}

/// IsMax can be true (generic Max move) or a string (species-specific G-Max move)
#[derive(Debug, Clone)]
pub enum IsMax {
    Generic,         // true
    Species(String), // Pokemon name like "Butterfree"
}

impl Serialize for IsMax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IsMax::Generic => serializer.serialize_bool(true),
            IsMax::Species(s) => serializer.serialize_str(s),
        }
    }
}

/// OHKO can be true (generic OHKO) or a string (type-based OHKO like "Ice")
#[derive(Debug, Clone)]
pub enum Ohko {
    Generic,           // true
    TypeBased(String), // Type name like "Ice", "Normal"
}

impl Serialize for Ohko {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Ohko::Generic => serializer.serialize_bool(true),
            Ohko::TypeBased(s) => serializer.serialize_str(s),
        }
    }
}

/// Multihit can be a single number or range [min, max]
#[derive(Debug, Clone)]
pub enum Multihit {
    Fixed(i32),
    Range(i32, i32),
}

impl<'de> Deserialize<'de> for Multihit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess, Visitor};

        struct MultihitVisitor;

        impl<'de> Visitor<'de> for MultihitVisitor {
            type Value = Multihit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number or array of two numbers")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Multihit::Fixed(value as i32))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let min: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let max: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Multihit::Range(min, max))
            }
        }

        deserializer.deserialize_any(MultihitVisitor)
    }
}

impl Serialize for Multihit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Multihit::Fixed(n) => serializer.serialize_i32(*n),
            Multihit::Range(min, max) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(min)?;
                seq.serialize_element(max)?;
                seq.end()
            }
        }
    }
}

/// StringOrVec can be a single string or an array of strings
#[derive(Debug, Clone, Default)]
pub enum StringOrVec {
    #[default]
    Empty,
    Single(String),
    Multiple(Vec<String>),
}

impl PartialEq<&str> for StringOrVec {
    fn eq(&self, other: &&str) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == *other,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == *other),
        }
    }
}

impl PartialEq<String> for StringOrVec {
    fn eq(&self, other: &String) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == other,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == other),
        }
    }
}

impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess, Visitor};

        struct StringOrVecVisitor;

        impl<'de> Visitor<'de> for StringOrVecVisitor {
            type Value = StringOrVec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or array of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrVec::Single(value.to_string()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(StringOrVec::Multiple(vec))
            }
        }

        deserializer.deserialize_any(StringOrVecVisitor)
    }
}

impl Serialize for StringOrVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrVec::Empty => serializer.serialize_none(),
            StringOrVec::Single(s) => serializer.serialize_str(s),
            StringOrVec::Multiple(v) => v.serialize(serializer),
        }
    }
}

impl StringOrVec {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            StringOrVec::Empty => Vec::new(),
            StringOrVec::Single(s) => vec![s.clone()],
            StringOrVec::Multiple(v) => v.clone(),
        }
    }

    pub fn contains(&self, value: &str) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == value,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == value),
        }
    }
}

impl Serialize for Accuracy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Accuracy::AlwaysHits => serializer.serialize_bool(true),
            Accuracy::Percent(p) => serializer.serialize_i32(*p),
        }
    }
}

/// Ability data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityData {
    #[serde(default)]
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "shortDesc", default)]
    pub short_desc: Option<String>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub flags: HashMap<String, i32>,
    #[serde(default)]
    pub effect_type: Option<String>,
}

/// Item data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    #[serde(default)]
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "isChoice", default)]
    pub is_choice: bool,
    #[serde(rename = "isBerry", default)]
    pub is_berry: bool,
    #[serde(rename = "isGem", default)]
    pub is_gem: bool,
    #[serde(default)]
    pub fling: Option<FlingData>,
    /// Natural Gift data (for berries)
    #[serde(rename = "naturalGift", default)]
    pub natural_gift: Option<serde_json::Value>,
    /// Type for Plate items (e.g., "Fire" for Flame Plate)
    #[serde(rename = "onPlate", default)]
    pub on_plate: Option<String>,
    /// Z-Move compatibility
    #[serde(rename = "zMove", default)]
    pub z_move: Option<serde_json::Value>,
    /// Mega Evolution stone target (e.g., "Froslass-Mega")
    #[serde(rename = "megaStone", default)]
    pub mega_stone: Option<StringOrVec>,
    /// Pokemon species that can use this mega stone (e.g., "Froslass")
    #[serde(rename = "megaEvolves", default)]
    pub mega_evolves: Option<StringOrVec>,
    /// Species that can use this item
    #[serde(rename = "itemUser", default)]
    pub item_user: Option<Vec<String>>,
    /// Stat boosts when item is used (e.g., for Cell Battery)
    #[serde(default)]
    pub boosts: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingData {
    #[serde(rename = "basePower", default)]
    pub base_power: i32,
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
}

/// Type effectiveness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeData {
    #[serde(rename = "damageTaken")]
    pub damage_taken: HashMap<String, u8>,
}

/// Ruleset data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetData {
    pub name: String,
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
}

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatureData {
    pub name: String,
    #[serde(default)]
    pub plus: Option<String>,
    #[serde(default)]
    pub minus: Option<String>,
}

/// Event data for learnsets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    #[serde(default)]
    pub generation: Option<u8>,
    #[serde(default)]
    pub level: Option<u8>,
    #[serde(default)]
    pub moves: Vec<String>,
    #[serde(default)]
    pub source: Option<String>,
}

/// Learnset data for a single species
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LearnsetData {
    /// Map from move ID to learn methods (e.g., "9M", "8L15", "7E")
    #[serde(default)]
    pub learnset: HashMap<String, Vec<String>>,
    /// Event-only moves
    #[serde(default)]
    pub event_data: Option<Vec<EventData>>,
    #[serde(default)]
    pub event_only: Option<bool>,
}

/// Format data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatData {
    pub name: String,
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
    #[serde(default)]
    pub team: Option<String>,
    #[serde(default)]
    pub game_type: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub rated: Option<serde_json::Value>, // can be bool or string
    #[serde(default)]
    pub search_show: Option<bool>,
    #[serde(default)]
    pub challenge_show: Option<bool>,
    #[serde(default)]
    pub tournament_show: Option<bool>,
    #[serde(default)]
    pub best_of_default: Option<bool>,
    #[serde(default)]
    pub ruleset: Vec<String>,
    #[serde(default)]
    pub banlist: Vec<String>,
    #[serde(default)]
    pub restricted: Vec<String>,
    #[serde(default)]
    pub unbanlist: Vec<String>,
    #[serde(default)]
    pub custom_rules: Option<Vec<String>>,
}

/// The main Dex structure
#[derive(Debug, Clone, Default)]
pub struct Dex {
    pub species: HashMap<ID, SpeciesData>,
    pub moves: HashMap<ID, MoveData>,
    pub abilities: HashMap<ID, AbilityData>,
    pub items: HashMap<ID, ItemData>,
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
}

/// Struct to hold JSON data for loading the Dex
pub struct DexJsonData<'a> {
    pub species_json: &'a str,
    pub moves_json: &'a str,
    pub abilities_json: &'a str,
    pub items_json: &'a str,
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

/// Embedded data for compile-time inclusion
pub mod embedded {
    pub const SPECIES_JSON: &str = include_str!("../data/species.json");
    pub const MOVES_JSON: &str = include_str!("../data/moves.json");
    pub const ABILITIES_JSON: &str = include_str!("../data/abilities.json");
    pub const ITEMS_JSON: &str = include_str!("../data/items.json");
    pub const TYPES_JSON: &str = include_str!("../data/typechart.json");
    pub const NATURES_JSON: &str = include_str!("../data/natures.json");
    pub const RULESETS_JSON: &str = include_str!("../data/rulesets.json");
    pub const ALIASES_JSON: &str = include_str!("../data/aliases.json");
    pub const COMPOUNDWORDNAMES_JSON: &str = include_str!("../data/compoundwordnames.json");
    pub const FORMATS_JSON: &str = include_str!("../data/formats.json");
    pub const FORMATS_DATA_JSON: &str = include_str!("../data/formats-data.json");
    pub const GEN8_PAST_JSON: &str = include_str!("../data/gen8-past.json");
}

impl Dex {
}

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
        let static_ability = dex.get_ability("Static").expect("Static not found");
        assert_eq!(static_ability.name, "Static");

        // Check items
        let leftovers = dex.items().get("Leftovers").expect("Leftovers not found");
        assert_eq!(leftovers.name, "Leftovers");
    }

    #[test]
    fn test_type_effectiveness() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Electric vs Water = super effective
        assert_eq!(dex.get_effectiveness("Electric", "Water"), 2.0);

        // Electric vs Ground = immune
        assert_eq!(dex.get_effectiveness("Electric", "Ground"), 0.0);

        // Electric vs Electric = not very effective
        assert_eq!(dex.get_effectiveness("Electric", "Electric"), 0.5);

        // Electric vs Normal = neutral
        assert_eq!(dex.get_effectiveness("Electric", "Normal"), 1.0);

        // Fighting vs Ghost = immune
        assert_eq!(dex.get_effectiveness("Fighting", "Ghost"), 0.0);

        // Fire vs Grass = super effective
        assert_eq!(dex.get_effectiveness("Fire", "Grass"), 2.0);
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
        let dex = Dex::load_default().expect("Failed to load dex");

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
