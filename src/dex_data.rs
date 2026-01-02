//! Dex Data - Core types and ID system
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains the core type definitions used throughout the simulator.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

/// An ID must be lowercase alphanumeric.
/// This is the core identifier type used throughout Pokemon Showdown.
/// JavaScript equivalent: ID (sim/global-types.ts)
/// JavaScript uses string type with lowercase alphanumeric normalization
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

/// Gender names
/// JavaScript equivalent: GenderName (sim/global-types.ts)
/// JavaScript: 'M' | 'F' | 'N'
/// JavaScript uses a lookup table: {M: "M", F: "F", N: "N"}
/// Any other value (like "Male", "Female", "None") falls through to randomization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    None,
}

// Custom deserializer to match JavaScript's behavior:
// Only "M", "F", "N" are recognized. Everything else becomes None (needs randomization)
impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            "N" => Gender::None,
            _ => {
                // JavaScript: genders[set.gender] returns undefined for "Male", "Female", "None"
                // This triggers randomization via battle.sample(['M', 'F'])
                eprintln!("[GENDER_DESERIALIZE] Unrecognized gender '{}', will need randomization", s);
                Gender::None
            }
        })
    }
}

impl Gender {
    pub fn parse(s: &str) -> Self {
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
/// JavaScript equivalent: StatID (sim/global-types.ts)
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

    pub fn parse(s: &str) -> Option<StatID> {
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
/// JavaScript equivalent: StatsTable (sim/global-types.ts)
/// 6 fields in JavaScript
pub struct StatsTable {
    /// Hit Points
    /// JavaScript: hp: number
    pub hp: i32,
    /// Attack
    /// JavaScript: atk: number
    pub atk: i32,
    /// Defense
    /// JavaScript: def: number
    pub def: i32,
    /// Special Attack
    /// JavaScript: spa: number
    pub spa: i32,
    /// Special Defense
    /// JavaScript: spd: number
    pub spd: i32,
    /// Speed
    /// JavaScript: spe: number
    pub spe: i32,
}

impl StatsTable {
    pub fn new(hp: i32, atk: i32, def: i32, spa: i32, spd: i32, spe: i32) -> Self {
        Self {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        }
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
/// JavaScript equivalent: BoostID (sim/global-types.ts)
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
/// JavaScript equivalent: BoostsTable (sim/global-types.ts)
/// 7 fields in JavaScript
pub struct BoostsTable {
    /// Attack boost (-6 to +6)
    /// JavaScript: atk: number
    pub atk: i8,
    /// Defense boost (-6 to +6)
    /// JavaScript: def: number
    pub def: i8,
    /// Special Attack boost (-6 to +6)
    /// JavaScript: spa: number
    pub spa: i8,
    /// Special Defense boost (-6 to +6)
    /// JavaScript: spd: number
    pub spd: i8,
    /// Speed boost (-6 to +6)
    /// JavaScript: spe: number
    pub spe: i8,
    /// Accuracy boost (-6 to +6)
    /// JavaScript: accuracy: number
    pub accuracy: i8,
    /// Evasion boost (-6 to +6)
    /// JavaScript: evasion: number
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
/// JavaScript equivalent: EffectType (sim/global-types.ts)
/// JavaScript uses string literals for effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EffectType {
    #[default]
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

/// Nonstandard classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// JavaScript equivalent: Nonstandard (sim/global-types.ts)
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
/// JavaScript equivalent: GameType (sim/global-types.ts)
/// JavaScript: 'singles' | 'doubles' | 'triples' | 'rotation' | 'multi' | 'freeforall'
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
/// JavaScript equivalent: SideID (sim/global-types.ts)
pub enum SideID {
    P1,
    P2,
    P3,
    P4,
}

impl SideID {
    pub fn parse(s: &str) -> Option<Self> {
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
/// JavaScript equivalent: MoveTarget (sim/global-types.ts)
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
/// JavaScript equivalent: EffectState (sim/pokemon.ts)
/// 3 fields in JavaScript: id, effectOrder, duration
pub struct EffectState {
    /// Effect ID
    /// JavaScript: id: string
    pub id: ID,

    /// Effect order for sorting
    /// JavaScript: effectOrder: number
    pub effect_order: i32,

    /// Duration in turns
    /// JavaScript: duration?: number
    pub duration: Option<i32>,

    // TODO: DELETE - Not in JavaScript EffectState (only has id, effectOrder, duration)
    // These fields may belong in a different Rust struct or be Rust-specific extensions
    /// Number of layers (for Spikes, Toxic Spikes, etc.)
    pub layers: Option<i32>,

    // TODO: DELETE - Not in JavaScript EffectState
    /// Source slot for slot conditions
    pub source_slot: Option<String>,

    // TODO: DELETE - Not in JavaScript EffectState
    /// Target side index
    pub target_side: Option<usize>,

    // TODO: DELETE - Not in JavaScript EffectState
    /// Custom data storage (flattened for serialization)
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl EffectState {
    pub fn new(id: ID) -> Self {
        Self {
            id,
            effect_order: 0,
            duration: None,
            layers: None,
            source_slot: None,
            target_side: None,
            data: HashMap::new(),
        }
    }

    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_layers(mut self, layers: i32) -> Self {
        self.layers = Some(layers);
        self
    }
}

/// Basic effect - base struct for abilities, items, moves, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: BasicEffect (sim/dex-data.ts)
/// 16 fields in JavaScript
pub struct BasicEffect {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: fullname: string
    pub fullname: String,
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// JavaScript: exists: boolean
    pub exists: bool,
    /// JavaScript: num: number
    pub num: i32,
    /// JavaScript: gen: number
    pub gen: u8,
    /// JavaScript: shortDesc: string
    pub short_desc: String,
    /// JavaScript: desc: string
    pub desc: String,
    /// JavaScript: isNonstandard?: Nonstandard
    pub is_nonstandard: Option<Nonstandard>,
    /// JavaScript: duration?: number
    pub duration: Option<i32>,
    /// JavaScript: noCopy: boolean
    pub no_copy: bool,
    /// JavaScript: affectsFainted: boolean
    pub affects_fainted: bool,
    /// JavaScript: status?: ID
    pub status: Option<ID>,
    /// JavaScript: weather?: ID
    pub weather: Option<ID>,
    /// JavaScript: sourceEffect: string
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

impl std::fmt::Display for BasicEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Nature (sim/global-types.ts)
pub struct Nature {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: plus?: StatIDExceptHP
    pub plus: Option<StatID>,
    /// JavaScript: minus?: StatIDExceptHP
    pub minus: Option<StatID>,
    /// JavaScript: gen: number
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

impl std::fmt::Display for Nature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Type info for type chart
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: TypeInfo (sim/global-types.ts)
pub struct TypeInfo {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// JavaScript: exists: boolean
    pub exists: bool,
    /// JavaScript: gen: number
    pub gen: u8,
    /// JavaScript: isNonstandard?: Nonstandard
    pub is_nonstandard: Option<Nonstandard>,
    /// Type chart: 0 = normal, 1 = weakness, 2 = resistance, 3 = immunity
    /// JavaScript: damageTaken: { [type: string]: number }
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
            Some(1) => 2.0, // Weakness
            Some(2) => 0.5, // Resistance
            Some(3) => 0.0, // Immunity
            _ => 1.0,       // Normal
        }
    }
}

impl std::fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// =========================================================================
// DEX NATURES - ported from dex-data.ts DexNatures class
// =========================================================================

/// Natures collection with caching
/// JavaScript equivalent: DexNatures (sim/dex-data.ts)
/// Fields: dex, allCache
pub struct DexNatures {
    gen: u8,
    nature_cache: HashMap<String, Nature>,
    all_cache: Option<Vec<Nature>>,
}

impl DexNatures {
    /// Create a new DexNatures for a given generation
    pub fn new(gen: u8) -> Self {
        Self {
            gen,
            nature_cache: HashMap::new(),
            all_cache: None,
        }
    }

    /// Get a nature by name or ID
    pub fn get(&mut self, name: &str) -> Nature {
        self.get_by_id(&to_id(name))
    }

    /// Get a nature by ID
    pub fn get_by_id(&mut self, id: &str) -> Nature {
        if id.is_empty() {
            return Nature {
                id: ID::empty(),
                name: String::new(),
                plus: None,
                minus: None,
                gen: 0,
            };
        }

        if let Some(nature) = self.nature_cache.get(id) {
            return nature.clone();
        }

        // Look up in nature data
        if let Some(nature) = get_nature_data(id) {
            let n = nature.clone();
            if n.gen > self.gen {
                // Mark as future if not available in this gen
            }
            self.nature_cache.insert(id.to_string(), n.clone());
            return n;
        }

        // Return non-existent nature
        Nature {
            id: ID::new(id),
            name: id.to_string(),
            plus: None,
            minus: None,
            gen: 0,
        }
    }

    /// Get all natures
    pub fn all(&mut self) -> Vec<Nature> {
        if let Some(ref all) = self.all_cache {
            return all.clone();
        }

        let natures = get_all_natures();
        self.all_cache = Some(natures.clone());
        natures
    }
}

/// Built-in nature data
fn get_nature_data(id: &str) -> Option<Nature> {
    let (plus, minus) = match id {
        "adamant" => (Some(StatID::Atk), Some(StatID::SpA)),
        "bashful" => (None, None),
        "bold" => (Some(StatID::Def), Some(StatID::Atk)),
        "brave" => (Some(StatID::Atk), Some(StatID::Spe)),
        "calm" => (Some(StatID::SpD), Some(StatID::Atk)),
        "careful" => (Some(StatID::SpD), Some(StatID::SpA)),
        "docile" => (None, None),
        "gentle" => (Some(StatID::SpD), Some(StatID::Def)),
        "hardy" => (None, None),
        "hasty" => (Some(StatID::Spe), Some(StatID::Def)),
        "impish" => (Some(StatID::Def), Some(StatID::SpA)),
        "jolly" => (Some(StatID::Spe), Some(StatID::SpA)),
        "lax" => (Some(StatID::Def), Some(StatID::SpD)),
        "lonely" => (Some(StatID::Atk), Some(StatID::Def)),
        "mild" => (Some(StatID::SpA), Some(StatID::Def)),
        "modest" => (Some(StatID::SpA), Some(StatID::Atk)),
        "naive" => (Some(StatID::Spe), Some(StatID::SpD)),
        "naughty" => (Some(StatID::Atk), Some(StatID::SpD)),
        "quiet" => (Some(StatID::SpA), Some(StatID::Spe)),
        "quirky" => (None, None),
        "rash" => (Some(StatID::SpA), Some(StatID::SpD)),
        "relaxed" => (Some(StatID::Def), Some(StatID::Spe)),
        "sassy" => (Some(StatID::SpD), Some(StatID::Spe)),
        "serious" => (None, None),
        "timid" => (Some(StatID::Spe), Some(StatID::Atk)),
        _ => return None,
    };

    let name = match id {
        "adamant" => "Adamant",
        "bashful" => "Bashful",
        "bold" => "Bold",
        "brave" => "Brave",
        "calm" => "Calm",
        "careful" => "Careful",
        "docile" => "Docile",
        "gentle" => "Gentle",
        "hardy" => "Hardy",
        "hasty" => "Hasty",
        "impish" => "Impish",
        "jolly" => "Jolly",
        "lax" => "Lax",
        "lonely" => "Lonely",
        "mild" => "Mild",
        "modest" => "Modest",
        "naive" => "Naive",
        "naughty" => "Naughty",
        "quiet" => "Quiet",
        "quirky" => "Quirky",
        "rash" => "Rash",
        "relaxed" => "Relaxed",
        "sassy" => "Sassy",
        "serious" => "Serious",
        "timid" => "Timid",
        _ => return None,
    };

    Some(Nature {
        id: ID::new(id),
        name: name.to_string(),
        plus,
        minus,
        gen: 3,
    })
}

/// Get all natures
fn get_all_natures() -> Vec<Nature> {
    let nature_ids = [
        "adamant", "bashful", "bold", "brave", "calm", "careful", "docile", "gentle", "hardy",
        "hasty", "impish", "jolly", "lax", "lonely", "mild", "modest", "naive", "naughty", "quiet",
        "quirky", "rash", "relaxed", "sassy", "serious", "timid",
    ];
    nature_ids
        .iter()
        .filter_map(|id| get_nature_data(id))
        .collect()
}

// =========================================================================
// DEX TYPES - ported from dex-data.ts DexTypes class
// =========================================================================

/// Types collection with caching
/// JavaScript equivalent: DexTypes (sim/dex-data.ts)
/// Fields: dex, allCache, namesCache
pub struct DexTypes {
    gen: u8,
    type_cache: HashMap<String, TypeInfo>,
    all_cache: Option<Vec<TypeInfo>>,
    names_cache: Option<Vec<String>>,
}

impl DexTypes {
    /// Create a new DexTypes for a given generation
    pub fn new(gen: u8) -> Self {
        Self {
            gen,
            type_cache: HashMap::new(),
            all_cache: None,
            names_cache: None,
        }
    }

    /// Get a type by name
    // TypeScript source:
    //
    //
    // 	get(name: string | Nature): Nature {
    // 		if (name && typeof name !== 'string') return name;
    // 		return this.getByID(toID(name));
    // 	}
    //
    pub fn get(&mut self, name: &str) -> TypeInfo {
        self.get_by_id(&to_id(name))
    }

    /// Get a type by ID
    // 	getByID(id: ID): Nature {
    // 		if (id === '') return EMPTY_NATURE;
    // 		let nature = this.natureCache.get(id);
    // 		if (nature) return nature;
    //
    // 		const alias = this.dex.getAlias(id);
    // 		if (alias) {
    // 			nature = this.get(alias);
    // 			if (nature.exists) {
    // 				this.natureCache.set(id, nature);
    // 			}
    // 			return nature;
    // 		}
    // 		if (id && this.dex.data.Natures.hasOwnProperty(id)) {
    // 			const natureData = this.dex.data.Natures[id];
    // 			nature = new Nature(natureData);
    // 			if (nature.gen > this.dex.gen) nature.isNonstandard = 'Future';
    // 		} else {
    // 			nature = new Nature({ name: id, exists: false });
    // 		}
    //
    // 		if (nature.exists) this.natureCache.set(id, this.dex.deepFreeze(nature));
    // 		return nature;
    // 	}
    //
    pub fn get_by_id(&mut self, id: &str) -> TypeInfo {
        if id.is_empty() {
            return TypeInfo {
                id: ID::empty(),
                name: String::new(),
                effect_type: EffectType::Condition,
                exists: false,
                gen: 0,
                is_nonstandard: None,
                damage_taken: HashMap::new(),
            };
        }

        if let Some(type_info) = self.type_cache.get(id) {
            return type_info.clone();
        }

        // Capitalize the name
        let type_name = capitalize_first(id);

        if let Some(type_info) = get_type_data(id, &type_name, self.gen) {
            self.type_cache.insert(id.to_string(), type_info.clone());
            return type_info;
        }

        // Return non-existent type
        TypeInfo {
            id: ID::new(id),
            name: type_name,
            effect_type: EffectType::Condition,
            exists: false,
            gen: 0,
            is_nonstandard: None,
            damage_taken: HashMap::new(),
        }
    }

    /// Get all type names (excluding nonstandard)
    pub fn names(&mut self) -> Vec<String> {
        if let Some(ref names) = self.names_cache {
            return names.clone();
        }

        let names: Vec<String> = self
            .all()
            .into_iter()
            .filter(|t| t.is_nonstandard.is_none())
            .map(|t| t.name)
            .collect();
        self.names_cache = Some(names.clone());
        names
    }

    /// Check if a name is a valid type name
    //
    // 	isName(name: string | null | undefined): boolean {
    // 		if (!name) return false;
    // 		const id = name.toLowerCase();
    // 		const typeName = id.charAt(0).toUpperCase() + id.substr(1);
    // 		return name === typeName && this.dex.data.TypeChart.hasOwnProperty(id);
    // 	}
    //
    pub fn is_name(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        let id = name.to_lowercase();
        let type_name = capitalize_first(&id);
        name == type_name && is_valid_type(&id)
    }

    /// Get all types
    //
    // 	all(): readonly Nature[] {
    // 		if (this.allCache) return this.allCache;
    // 		const natures = [];
    // 		for (const id in this.dex.data.Natures) {
    // 			natures.push(this.getByID(id as ID));
    // 		}
    // 		this.allCache = Object.freeze(natures);
    // 		return this.allCache;
    // 	}
    //
    pub fn all(&mut self) -> Vec<TypeInfo> {
        if let Some(ref all) = self.all_cache {
            return all.clone();
        }

        let types = get_all_types(self.gen);
        self.all_cache = Some(types.clone());
        types
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn is_valid_type(id: &str) -> bool {
    matches!(
        id,
        "normal"
            | "fire"
            | "water"
            | "electric"
            | "grass"
            | "ice"
            | "fighting"
            | "poison"
            | "ground"
            | "flying"
            | "psychic"
            | "bug"
            | "rock"
            | "ghost"
            | "dragon"
            | "dark"
            | "steel"
            | "fairy"
    )
}

fn get_type_data(id: &str, name: &str, gen: u8) -> Option<TypeInfo> {
    // Build damage_taken based on type chart
    let damage_taken = get_type_chart(id)?;

    // Check if type exists in this gen
    let (type_gen, is_nonstandard) = match id {
        "dark" | "steel" => (
            2,
            if gen < 2 {
                Some(Nonstandard::Future)
            } else {
                None
            },
        ),
        "fairy" => (
            6,
            if gen < 6 {
                Some(Nonstandard::Future)
            } else {
                None
            },
        ),
        _ => (1, None),
    };

    Some(TypeInfo {
        id: ID::new(id),
        name: name.to_string(),
        effect_type: EffectType::Condition,
        exists: true,
        gen: type_gen,
        is_nonstandard,
        damage_taken,
    })
}

fn get_type_chart(id: &str) -> Option<HashMap<String, u8>> {
    // 0 = normal, 1 = weakness, 2 = resistance, 3 = immunity
    let mut chart = HashMap::new();

    match id {
        "normal" => {
            chart.insert("fighting".to_string(), 1);
            chart.insert("ghost".to_string(), 3);
        }
        "fire" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 1);
            chart.insert("steel".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "water" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 1);
            chart.insert("grass".to_string(), 1);
            chart.insert("ice".to_string(), 2);
            chart.insert("steel".to_string(), 2);
        }
        "electric" => {
            chart.insert("electric".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("steel".to_string(), 2);
        }
        "grass" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 2);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("poison".to_string(), 1);
            chart.insert("ground".to_string(), 2);
            chart.insert("flying".to_string(), 1);
            chart.insert("bug".to_string(), 1);
        }
        "ice" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("ice".to_string(), 2);
            chart.insert("fighting".to_string(), 1);
            chart.insert("rock".to_string(), 1);
            chart.insert("steel".to_string(), 1);
        }
        "fighting" => {
            chart.insert("flying".to_string(), 1);
            chart.insert("psychic".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 2);
            chart.insert("dark".to_string(), 2);
            chart.insert("fairy".to_string(), 1);
        }
        "poison" => {
            chart.insert("grass".to_string(), 2);
            chart.insert("fighting".to_string(), 2);
            chart.insert("poison".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("psychic".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "ground" => {
            chart.insert("water".to_string(), 1);
            chart.insert("electric".to_string(), 3);
            chart.insert("grass".to_string(), 1);
            chart.insert("ice".to_string(), 1);
            chart.insert("poison".to_string(), 2);
            chart.insert("rock".to_string(), 2);
        }
        "flying" => {
            chart.insert("electric".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("fighting".to_string(), 2);
            chart.insert("ground".to_string(), 3);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 1);
        }
        "psychic" => {
            chart.insert("fighting".to_string(), 2);
            chart.insert("psychic".to_string(), 2);
            chart.insert("bug".to_string(), 1);
            chart.insert("ghost".to_string(), 1);
            chart.insert("dark".to_string(), 1);
        }
        "bug" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("fighting".to_string(), 2);
            chart.insert("ground".to_string(), 2);
            chart.insert("flying".to_string(), 1);
            chart.insert("rock".to_string(), 1);
        }
        "rock" => {
            chart.insert("normal".to_string(), 2);
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 1);
            chart.insert("grass".to_string(), 1);
            chart.insert("fighting".to_string(), 1);
            chart.insert("poison".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("steel".to_string(), 1);
        }
        "ghost" => {
            chart.insert("normal".to_string(), 3);
            chart.insert("fighting".to_string(), 3);
            chart.insert("poison".to_string(), 2);
            chart.insert("bug".to_string(), 2);
            chart.insert("ghost".to_string(), 1);
            chart.insert("dark".to_string(), 1);
        }
        "dragon" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 2);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("dragon".to_string(), 1);
            chart.insert("fairy".to_string(), 1);
        }
        "dark" => {
            chart.insert("fighting".to_string(), 1);
            chart.insert("psychic".to_string(), 3);
            chart.insert("bug".to_string(), 1);
            chart.insert("ghost".to_string(), 2);
            chart.insert("dark".to_string(), 2);
            chart.insert("fairy".to_string(), 1);
        }
        "steel" => {
            chart.insert("normal".to_string(), 2);
            chart.insert("fire".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 2);
            chart.insert("fighting".to_string(), 1);
            chart.insert("poison".to_string(), 3);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("psychic".to_string(), 2);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 2);
            chart.insert("dragon".to_string(), 2);
            chart.insert("steel".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "fairy" => {
            chart.insert("fighting".to_string(), 2);
            chart.insert("poison".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("dragon".to_string(), 3);
            chart.insert("dark".to_string(), 2);
            chart.insert("steel".to_string(), 1);
        }
        _ => return None,
    }

    Some(chart)
}

fn get_all_types(gen: u8) -> Vec<TypeInfo> {
    let type_ids = [
        "normal", "fire", "water", "electric", "grass", "ice", "fighting", "poison", "ground",
        "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
    ];

    type_ids
        .iter()
        .filter_map(|id| {
            let name = capitalize_first(id);
            get_type_data(id, &name, gen)
        })
        .filter(|t| t.gen <= gen)
        .collect()
}

// =========================================================================
// DEX STATS - ported from dex-data.ts DexStats class
// =========================================================================

/// Stats helper with name lookups
/// JavaScript equivalent: DexStats (sim/dex-data.ts)
/// 9 fields in JavaScript
pub struct DexStats {
    gen: u8,
}

impl DexStats {
    /// Create a new DexStats for a given generation
    pub fn new(gen: u8) -> Self {
        Self { gen }
    }

    /// Get short stat names
    pub fn short_names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Atk");
            map.insert(StatID::Def, "Def");
            map.insert(StatID::SpA, "SpA");
            map.insert(StatID::SpD, "SpD");
            map.insert(StatID::Spe, "Spe");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Atk");
            map.insert(StatID::Def, "Def");
            map.insert(StatID::SpA, "Spc");
            map.insert(StatID::SpD, "[SpD]");
            map.insert(StatID::Spe, "Spe");
        }
        map
    }

    /// Get medium stat names
    pub fn medium_names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Sp. Atk");
            map.insert(StatID::SpD, "Sp. Def");
            map.insert(StatID::Spe, "Speed");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special");
            map.insert(StatID::SpD, "[Sp. Def]");
            map.insert(StatID::Spe, "Speed");
        }
        map
    }

    /// Get full stat names
    //
    // 	names(): readonly string[] {
    // 		if (this.namesCache) return this.namesCache;
    //
    // 		this.namesCache = this.all().filter(type => !type.isNonstandard).map(type => type.name);
    //
    // 		return this.namesCache;
    // 	}
    //
    pub fn names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special Attack");
            map.insert(StatID::SpD, "Special Defense");
            map.insert(StatID::Spe, "Speed");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special");
            map.insert(StatID::SpD, "[Special Defense]");
            map.insert(StatID::Spe, "Speed");
        }
        map
    }

    /// Get stat ID from name
    // 	getID(name: string) {
    // 		if (name === 'Spd') return 'spe' as StatID;
    // 		const id = toID(name);
    // 		if (reverseCache[id]) return reverseCache[id];
    // 		if (idsCache.includes(id as StatID)) return id as StatID;
    // 		return null;
    // 	}
    //
    pub fn get_id(&self, name: &str) -> Option<StatID> {
        // Special case: "Spd" means Speed, not Special Defense
        if name == "Spd" {
            return Some(StatID::Spe);
        }
        StatID::parse(name)
    }

    /// Get all stat IDs
    // 	ids(): typeof idsCache {
    // 		return idsCache;
    // 	}
    //
    pub fn ids(&self) -> &'static [StatID] {
        StatID::all()
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
