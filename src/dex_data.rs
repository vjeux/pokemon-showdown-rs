//! Dex Data - Core types and ID system
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the core type definitions used throughout the simulator.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An ID must be lowercase alphanumeric.
/// This is the core identifier type used throughout Pokemon Showdown.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ID(String);

impl ID {
    /// Create a new ID from a string, converting to lowercase and removing non-alphanumeric chars
    pub fn new(text: &str) -> Self {
        ID(to_id(text))
    }

    /// Create an empty ID
    pub fn empty() -> Self {
        ID(String::new())
    }

    /// Check if the ID is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ID {
    fn from(s: &str) -> Self {
        ID::new(s)
    }
}

impl From<String> for ID {
    fn from(s: String) -> Self {
        ID::new(&s)
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Converts anything to an ID. An ID must have only lowercase alphanumeric characters.
pub fn to_id(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Gender names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    None,
}

impl Gender {
    pub fn from_str(s: &str) -> Self {
        match s {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Gender::Male => "M",
            Gender::Female => "F",
            Gender::None => "",
        }
    }
}

/// Stat IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatID {
    HP,
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
}

impl StatID {
    pub fn all() -> &'static [StatID] {
        &[
            StatID::HP,
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ]
    }

    pub fn except_hp() -> &'static [StatID] {
        &[
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ]
    }

    pub fn from_str(s: &str) -> Option<StatID> {
        match s.to_lowercase().as_str() {
            "hp" | "hitpoints" => Some(StatID::HP),
            "atk" | "attack" => Some(StatID::Atk),
            "def" | "defense" => Some(StatID::Def),
            "spa" | "spatk" | "specialattack" | "special" | "spc" => Some(StatID::SpA),
            "spd" | "spdef" | "specialdefense" => Some(StatID::SpD),
            "spe" | "speed" => Some(StatID::Spe),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            StatID::HP => "hp",
            StatID::Atk => "atk",
            StatID::Def => "def",
            StatID::SpA => "spa",
            StatID::SpD => "spd",
            StatID::Spe => "spe",
        }
    }
}

/// Stats table (all 6 stats)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StatsTable {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

impl StatsTable {
    pub fn new(hp: i32, atk: i32, def: i32, spa: i32, spd: i32, spe: i32) -> Self {
        Self { hp, atk, def, spa, spd, spe }
    }

    pub fn get(&self, stat: StatID) -> i32 {
        match stat {
            StatID::HP => self.hp,
            StatID::Atk => self.atk,
            StatID::Def => self.def,
            StatID::SpA => self.spa,
            StatID::SpD => self.spd,
            StatID::Spe => self.spe,
        }
    }

    pub fn set(&mut self, stat: StatID, value: i32) {
        match stat {
            StatID::HP => self.hp = value,
            StatID::Atk => self.atk = value,
            StatID::Def => self.def = value,
            StatID::SpA => self.spa = value,
            StatID::SpD => self.spd = value,
            StatID::Spe => self.spe = value,
        }
    }
}

/// Boost IDs (stats that can be boosted, plus accuracy/evasion)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoostID {
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
    Accuracy,
    Evasion,
}

impl BoostID {
    pub fn all() -> &'static [BoostID] {
        &[
            BoostID::Atk,
            BoostID::Def,
            BoostID::SpA,
            BoostID::SpD,
            BoostID::Spe,
            BoostID::Accuracy,
            BoostID::Evasion,
        ]
    }

    pub fn stats_only() -> &'static [BoostID] {
        &[
            BoostID::Atk,
            BoostID::Def,
            BoostID::SpA,
            BoostID::SpD,
            BoostID::Spe,
        ]
    }
}

/// Boosts table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BoostsTable {
    pub atk: i8,
    pub def: i8,
    pub spa: i8,
    pub spd: i8,
    pub spe: i8,
    pub accuracy: i8,
    pub evasion: i8,
}

impl BoostsTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, boost: BoostID) -> i8 {
        match boost {
            BoostID::Atk => self.atk,
            BoostID::Def => self.def,
            BoostID::SpA => self.spa,
            BoostID::SpD => self.spd,
            BoostID::Spe => self.spe,
            BoostID::Accuracy => self.accuracy,
            BoostID::Evasion => self.evasion,
        }
    }

    pub fn set(&mut self, boost: BoostID, value: i8) {
        let clamped = value.clamp(-6, 6);
        match boost {
            BoostID::Atk => self.atk = clamped,
            BoostID::Def => self.def = clamped,
            BoostID::SpA => self.spa = clamped,
            BoostID::SpD => self.spd = clamped,
            BoostID::Spe => self.spe = clamped,
            BoostID::Accuracy => self.accuracy = clamped,
            BoostID::Evasion => self.evasion = clamped,
        }
    }

    pub fn boost(&mut self, boost: BoostID, amount: i8) -> i8 {
        let old = self.get(boost);
        let new = (old + amount).clamp(-6, 6);
        self.set(boost, new);
        new - old // Return actual change
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Effect type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    Condition,
    Pokemon,
    Move,
    Item,
    Ability,
    Format,
    Nature,
    Ruleset,
    Weather,
    Status,
    Terrain,
    Rule,
    ValidatorRule,
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Condition
    }
}

/// Nonstandard classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nonstandard {
    Past,
    Future,
    Unobtainable,
    CAP,
    LGPE,
    Custom,
    Gigantamax,
}

/// Game type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameType {
    #[default]
    Singles,
    Doubles,
    Triples,
    Rotation,
    Multi,
    FreeForAll,
}

/// Side ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SideID {
    P1,
    P2,
    P3,
    P4,
}

impl SideID {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "p1" => Some(SideID::P1),
            "p2" => Some(SideID::P2),
            "p3" => Some(SideID::P3),
            "p4" => Some(SideID::P4),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            SideID::P1 => "p1",
            SideID::P2 => "p2",
            SideID::P3 => "p3",
            SideID::P4 => "p4",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            SideID::P1 => 0,
            SideID::P2 => 1,
            SideID::P3 => 2,
            SideID::P4 => 3,
        }
    }
}

/// Move target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MoveTarget {
    #[default]
    Normal,
    Self_,
    AdjacentAlly,
    AdjacentAllyOrSelf,
    AdjacentFoe,
    AllAdjacent,
    AllAdjacentFoes,
    Allies,
    AllySide,
    AllyTeam,
    Any,
    FoeSide,
    RandomNormal,
    Scripted,
    All,
}

/// Effect state - stores runtime data about effects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EffectState {
    pub id: ID,
    pub effect_order: u32,
    pub duration: Option<u32>,
    pub source_slot: Option<String>,
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl EffectState {
    pub fn new(id: ID) -> Self {
        Self {
            id,
            effect_order: 0,
            duration: None,
            source_slot: None,
            data: HashMap::new(),
        }
    }

    pub fn with_duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Basic effect - base struct for abilities, items, moves, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicEffect {
    pub id: ID,
    pub name: String,
    pub fullname: String,
    pub effect_type: EffectType,
    pub exists: bool,
    pub num: i32,
    pub gen: u8,
    pub short_desc: String,
    pub desc: String,
    pub is_nonstandard: Option<Nonstandard>,
    pub duration: Option<u32>,
    pub no_copy: bool,
    pub affects_fainted: bool,
    pub status: Option<ID>,
    pub weather: Option<ID>,
    pub source_effect: String,
}

impl Default for BasicEffect {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            fullname: String::new(),
            effect_type: EffectType::Condition,
            exists: false,
            num: 0,
            gen: 0,
            short_desc: String::new(),
            desc: String::new(),
            is_nonstandard: None,
            duration: None,
            no_copy: false,
            affects_fainted: false,
            status: None,
            weather: None,
            source_effect: String::new(),
        }
    }
}

impl BasicEffect {
    pub fn new(name: &str) -> Self {
        let id = ID::new(name);
        Self {
            id: id.clone(),
            name: name.to_string(),
            fullname: name.to_string(),
            exists: !id.is_empty(),
            ..Default::default()
        }
    }
}

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nature {
    pub id: ID,
    pub name: String,
    pub plus: Option<StatID>,
    pub minus: Option<StatID>,
    pub gen: u8,
}

impl Nature {
    pub fn new(name: &str, plus: Option<StatID>, minus: Option<StatID>) -> Self {
        Self {
            id: ID::new(name),
            name: name.to_string(),
            plus,
            minus,
            gen: 3,
        }
    }

    /// Get the stat modifier for a given stat (1.0, 1.1, or 0.9)
    pub fn stat_modifier(&self, stat: StatID) -> f64 {
        if stat == StatID::HP {
            return 1.0;
        }
        let stat_boost = match stat {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => return 1.0,
        };
        let plus_match = self.plus.map(|s| match s {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => BoostID::Atk,
        });
        let minus_match = self.minus.map(|s| match s {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => BoostID::Atk,
        });

        if plus_match == Some(stat_boost) {
            1.1
        } else if minus_match == Some(stat_boost) {
            0.9
        } else {
            1.0
        }
    }
}

/// Type info for type chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub id: ID,
    pub name: String,
    pub effect_type: EffectType,
    pub exists: bool,
    pub gen: u8,
    pub is_nonstandard: Option<Nonstandard>,
    /// Type chart: 0 = normal, 1 = weakness, 2 = resistance, 3 = immunity
    pub damage_taken: HashMap<String, u8>,
}

impl TypeInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: ID::new(name),
            name: name.to_string(),
            effect_type: EffectType::Condition,
            exists: true,
            gen: 1,
            is_nonstandard: None,
            damage_taken: HashMap::new(),
        }
    }

    /// Get the damage multiplier when attacked by a move of the given type
    pub fn damage_multiplier(&self, attacking_type: &str) -> f64 {
        match self.damage_taken.get(attacking_type) {
            Some(1) => 2.0,  // Weakness
            Some(2) => 0.5,  // Resistance
            Some(3) => 0.0,  // Immunity
            _ => 1.0,        // Normal
        }
    }
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
}
