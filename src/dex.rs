//! Dex - Pokemon Showdown Data System
//!
//! Handles getting data about Pokemon, items, moves, abilities, etc.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, StatsTable};

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
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spa: u32,
    pub spd: u32,
    pub spe: u32,
}

impl From<BaseStatsData> for StatsTable {
    fn from(data: BaseStatsData) -> Self {
        StatsTable {
            hp: data.hp as i32,
            atk: data.atk as i32,
            def: data.def as i32,
            spa: data.spa as i32,
            spd: data.spd as i32,
            spe: data.spe as i32,
        }
    }
}

/// Species data from the pokedex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesData {
    pub num: i32,
    pub name: String,
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
    pub gender_ratio: Option<GenderRatio>,
    #[serde(default)]
    pub evos: Vec<String>,
    #[serde(default)]
    pub prevo: Option<String>,
    #[serde(default)]
    pub evo_level: Option<u32>,
    #[serde(default)]
    pub base_species: Option<String>,
    #[serde(default)]
    pub forme: Option<String>,
    #[serde(default)]
    pub other_formes: Vec<String>,
}

/// Move secondary effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSecondary {
    #[serde(default)]
    pub chance: Option<u32>,
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
    pub num: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub category: String,
    #[serde(default)]
    pub base_power: u32,
    #[serde(default, deserialize_with = "deserialize_accuracy")]
    pub accuracy: Accuracy,
    #[serde(default)]
    pub pp: u32,
    #[serde(default)]
    pub priority: i8,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub secondary: Option<MoveSecondary>,
    #[serde(default)]
    pub flags: HashMap<String, u32>,
    #[serde(default)]
    pub boosts: Option<HashMap<String, i32>>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
    #[serde(default)]
    pub drain: Option<(u32, u32)>,
    #[serde(default)]
    pub recoil: Option<(u32, u32)>,
    #[serde(default)]
    pub heal: Option<(u32, u32)>,
    #[serde(default)]
    pub multihit: Option<Multihit>,
    #[serde(rename = "isZ", default)]
    pub is_z: Option<String>,
    #[serde(rename = "isMax", default)]
    pub is_max: Option<bool>,
}

/// Accuracy can be a number or true (always hits)
#[derive(Debug, Clone)]
pub enum Accuracy {
    Percent(u32),
    AlwaysHits,
}

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy::Percent(100)
    }
}

fn deserialize_accuracy<'de, D>(deserializer: D) -> Result<Accuracy, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct AccuracyVisitor;

    impl<'de> Visitor<'de> for AccuracyVisitor {
        type Value = Accuracy;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or boolean")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value {
                Ok(Accuracy::AlwaysHits)
            } else {
                Ok(Accuracy::Percent(0))
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Accuracy::Percent(value as u32))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Accuracy::Percent(value as u32))
        }
    }

    deserializer.deserialize_any(AccuracyVisitor)
}

/// Multihit can be a single number or range [min, max]
#[derive(Debug, Clone)]
pub enum Multihit {
    Fixed(u32),
    Range(u32, u32),
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
                Ok(Multihit::Fixed(value as u32))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let min: u32 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let max: u32 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
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
            Multihit::Fixed(n) => serializer.serialize_u32(*n),
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

impl Serialize for Accuracy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Accuracy::AlwaysHits => serializer.serialize_bool(true),
            Accuracy::Percent(p) => serializer.serialize_u32(*p),
        }
    }
}

/// Ability data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityData {
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "shortDesc", default)]
    pub short_desc: Option<String>,
    #[serde(default)]
    pub rating: Option<f64>,
}

/// Item data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub num: i32,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(rename = "isChoice", default)]
    pub is_choice: bool,
    #[serde(default)]
    pub fling: Option<FlingData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingData {
    #[serde(rename = "basePower", default)]
    pub base_power: u32,
}

/// Type effectiveness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeData {
    #[serde(rename = "damageTaken")]
    pub damage_taken: HashMap<String, u8>,
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

/// The main Dex structure
#[derive(Debug, Clone, Default)]
pub struct Dex {
    pub species: HashMap<ID, SpeciesData>,
    pub moves: HashMap<ID, MoveData>,
    pub abilities: HashMap<ID, AbilityData>,
    pub items: HashMap<ID, ItemData>,
    pub types: HashMap<String, TypeData>,
    pub natures: HashMap<ID, NatureData>,
    pub gen: u8,
}

impl Dex {
    /// Create a new Dex for a specific generation
    pub fn new(gen: u8) -> Self {
        Self {
            species: HashMap::new(),
            moves: HashMap::new(),
            abilities: HashMap::new(),
            items: HashMap::new(),
            types: HashMap::new(),
            natures: HashMap::new(),
            gen,
        }
    }

    /// Load data from JSON strings
    pub fn load_from_json(
        species_json: &str,
        moves_json: &str,
        abilities_json: &str,
        items_json: &str,
        types_json: &str,
        natures_json: &str,
    ) -> Result<Self, serde_json::Error> {
        let species_raw: HashMap<String, SpeciesData> = serde_json::from_str(species_json)?;
        let moves_raw: HashMap<String, MoveData> = serde_json::from_str(moves_json)?;
        let abilities_raw: HashMap<String, AbilityData> = serde_json::from_str(abilities_json)?;
        let items_raw: HashMap<String, ItemData> = serde_json::from_str(items_json)?;
        let types: HashMap<String, TypeData> = serde_json::from_str(types_json)?;
        let natures_raw: HashMap<String, NatureData> = serde_json::from_str(natures_json)?;

        // Convert string keys to ID keys
        let species = species_raw.into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let moves = moves_raw.into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let abilities = abilities_raw.into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let items = items_raw.into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let natures = natures_raw.into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();

        Ok(Self {
            species,
            moves,
            abilities,
            items,
            types,
            natures,
            gen: 9, // Default to gen 9
        })
    }

    /// Get species data by name or ID
    pub fn get_species(&self, name: &str) -> Option<&SpeciesData> {
        let id = ID::new(name);
        self.species.get(&id)
    }

    /// Get move data by name or ID
    pub fn get_move(&self, name: &str) -> Option<&MoveData> {
        let id = ID::new(name);
        self.moves.get(&id)
    }

    /// Get ability data by name or ID
    pub fn get_ability(&self, name: &str) -> Option<&AbilityData> {
        let id = ID::new(name);
        self.abilities.get(&id)
    }

    /// Get item data by name or ID
    pub fn get_item(&self, name: &str) -> Option<&ItemData> {
        let id = ID::new(name);
        self.items.get(&id)
    }

    /// Get type data by name
    pub fn get_type(&self, name: &str) -> Option<&TypeData> {
        // Types use capitalized names as keys
        self.types.get(name)
    }

    /// Get nature data by name or ID
    pub fn get_nature(&self, name: &str) -> Option<&NatureData> {
        let id = ID::new(name);
        self.natures.get(&id)
    }

    /// Get type effectiveness multiplier
    /// 0 = immune (0x), 1 = not very effective (0.5x), 2 = neutral (1x), 3 = super effective (2x)
    /// Returns the numeric multiplier
    pub fn get_effectiveness(&self, attack_type: &str, defend_type: &str) -> f64 {
        if let Some(type_data) = self.get_type(defend_type) {
            if let Some(&effectiveness) = type_data.damage_taken.get(attack_type) {
                return match effectiveness {
                    0 => 1.0,  // Neutral
                    1 => 2.0,  // Super effective
                    2 => 0.5,  // Not very effective
                    3 => 0.0,  // Immune
                    _ => 1.0,
                };
            }
        }
        1.0 // Default to neutral
    }

    /// Get total type effectiveness against a Pokemon's types
    pub fn get_type_effectiveness(&self, attack_type: &str, defend_types: &[String]) -> f64 {
        let mut mult = 1.0;
        for defend_type in defend_types {
            mult *= self.get_effectiveness(attack_type, defend_type);
        }
        mult
    }

    /// Check if a move has a specific flag
    pub fn move_has_flag(&self, move_name: &str, flag: &str) -> bool {
        if let Some(move_data) = self.get_move(move_name) {
            move_data.flags.contains_key(flag)
        } else {
            false
        }
    }

    /// Get base stats for a species
    pub fn get_base_stats(&self, species_name: &str) -> Option<StatsTable> {
        self.get_species(species_name)
            .map(|s| s.base_stats.clone().into())
    }
}

/// Embedded data for compile-time inclusion
pub mod embedded {
    pub const SPECIES_JSON: &str = include_str!("../data/species.json");
    pub const MOVES_JSON: &str = include_str!("../data/moves.json");
    pub const ABILITIES_JSON: &str = include_str!("../data/abilities.json");
    pub const ITEMS_JSON: &str = include_str!("../data/items.json");
    pub const TYPES_JSON: &str = include_str!("../data/typechart.json");
    pub const NATURES_JSON: &str = include_str!("../data/natures.json");
}

impl Dex {
    /// Load the embedded default data
    pub fn load_default() -> Result<Self, serde_json::Error> {
        Self::load_from_json(
            embedded::SPECIES_JSON,
            embedded::MOVES_JSON,
            embedded::ABILITIES_JSON,
            embedded::ITEMS_JSON,
            embedded::TYPES_JSON,
            embedded::NATURES_JSON,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dex() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Check species
        let pikachu = dex.get_species("Pikachu").expect("Pikachu not found");
        assert_eq!(pikachu.name, "Pikachu");
        assert_eq!(pikachu.types, vec!["Electric"]);
        assert_eq!(pikachu.base_stats.hp, 35);
        assert_eq!(pikachu.base_stats.spe, 90);

        // Check moves
        let thunderbolt = dex.get_move("Thunderbolt").expect("Thunderbolt not found");
        assert_eq!(thunderbolt.name, "Thunderbolt");
        assert_eq!(thunderbolt.move_type, "Electric");
        assert_eq!(thunderbolt.base_power, 90);

        // Check abilities
        let static_ability = dex.get_ability("Static").expect("Static not found");
        assert_eq!(static_ability.name, "Static");

        // Check items
        let leftovers = dex.get_item("Leftovers").expect("Leftovers not found");
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

        // Electric vs Water/Flying = 4x
        let types = vec!["Water".to_string(), "Flying".to_string()];
        assert_eq!(dex.get_type_effectiveness("Electric", &types), 4.0);

        // Ground vs Electric/Flying = 0x (Flying is immune to Ground)
        let types = vec!["Electric".to_string(), "Flying".to_string()];
        assert_eq!(dex.get_type_effectiveness("Ground", &types), 0.0);
    }

    #[test]
    fn test_move_flags() {
        let dex = Dex::load_default().expect("Failed to load dex");

        assert!(dex.move_has_flag("Thunderbolt", "protect"));
        assert!(dex.move_has_flag("Quick Attack", "contact"));
        assert!(!dex.move_has_flag("Thunderbolt", "contact"));
    }

    #[test]
    fn test_natures() {
        let dex = Dex::load_default().expect("Failed to load dex");

        let adamant = dex.get_nature("Adamant").expect("Adamant not found");
        assert_eq!(adamant.name, "Adamant");
        assert_eq!(adamant.plus.as_deref(), Some("atk"));
        assert_eq!(adamant.minus.as_deref(), Some("spa"));

        let hardy = dex.get_nature("Hardy").expect("Hardy not found");
        assert!(hardy.plus.is_none()); // Neutral nature
        assert!(hardy.minus.is_none());
    }
}
