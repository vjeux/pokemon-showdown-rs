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
pub use default_true::default_true;
pub use default_crit_ratio::default_crit_ratio;
pub use deserialize_accuracy::deserialize_accuracy;
pub use deserialize_is_max::deserialize_is_max;
pub use deserialize_ohko::deserialize_ohko;

/// Gender ratio structure
/// Gender ratio (chance of male vs female)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// JavaScript equivalent: GenderRatio (sim/dex-species.ts)
/// 2 fields in JavaScript
pub struct GenderRatio {
    /// Probability of male (0.0 to 1.0)
    /// JavaScript: M: number
    #[serde(default)]
    pub m: f64,
    /// Probability of female (0.0 to 1.0)
    /// JavaScript: F: number
    #[serde(default)]
    pub f: f64,
}

/// Abilities structure (mapping slot to ability name)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: Abilities (sim/dex-species.ts)
/// 4 fields in JavaScript
pub struct AbilitySlots {
    /// Regular ability slot 0
    /// JavaScript: "0": string
    #[serde(rename = "0", default)]
    pub slot0: Option<String>,
    /// Regular ability slot 1 (second ability)
    /// JavaScript: "1"?: string
    #[serde(rename = "1", default)]
    pub slot1: Option<String>,
    /// Hidden ability
    /// JavaScript: "H"?: string
    #[serde(rename = "H", default)]
    pub hidden: Option<String>,
    /// Special ability (event-only)
    /// JavaScript: "S"?: string
    #[serde(rename = "S", default)]
    pub special: Option<String>,
}

/// Base stats as stored in JSON
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: StatsTable (sim/global-types.ts)
/// 6 fields in JavaScript
pub struct BaseStatsData {
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

/// Pokemon species data from the Dex
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: SpeciesData (sim/dex-species.ts)
/// 50 fields in Rust (matching JavaScript fields)
pub struct SpeciesData {
    /// Species number in the Pokedex
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Species name
    /// JavaScript: name: string
    pub name: String,
    /// Type(s)
    /// JavaScript: types: string[]
    #[serde(default)]
    pub types: Vec<String>,
    /// Base stats
    /// JavaScript: baseStats: StatsTable
    #[serde(default)]
    pub base_stats: BaseStatsData,
    /// Max HP override (e.g., Shedinja always has 1 HP)
    /// JavaScript: maxHP?: number
    #[serde(rename = "maxHP", default)]
    pub max_hp: Option<i32>,
    /// Abilities
    /// JavaScript: abilities: { 0: string, 1?: string, H?: string, S?: string }
    #[serde(default)]
    pub abilities: AbilitySlots,
    /// Height in meters
    /// JavaScript: heightm: number
    #[serde(default)]
    pub heightm: f64,
    /// Weight in kilograms
    /// JavaScript: weightkg: number
    #[serde(default)]
    pub weightkg: f64,
    /// Color
    /// JavaScript: color?: string
    #[serde(default)]
    pub color: Option<String>,
    /// Gender
    /// JavaScript: gender?: 'M' | 'F' | 'N'
    /// TODO: Rust uses Option<String>, JavaScript uses literal union type
    #[serde(default)]
    pub gender: Option<String>,
    /// Gender ratio
    /// JavaScript: genderRatio?: { M: number, F: number }
    #[serde(default)]
    pub gender_ratio: Option<GenderRatio>,
    /// Evolutions
    /// JavaScript: evos: string[]
    #[serde(default)]
    pub evos: Vec<String>,
    /// Pre-evolution
    /// JavaScript: prevo?: string
    #[serde(default)]
    pub prevo: Option<String>,
    /// Evolution type
    /// JavaScript: evoType?: string
    #[serde(rename = "evoType", default)]
    pub evo_type: Option<String>,
    /// Evolution level
    /// JavaScript: evoLevel?: number
    #[serde(default)]
    pub evo_level: Option<i32>,
    /// Evolution item required
    /// JavaScript: evoItem?: string
    #[serde(rename = "evoItem", default)]
    pub evo_item: Option<String>,
    /// Evolution condition (e.g., "during the day")
    /// JavaScript: evoCondition?: string
    #[serde(rename = "evoCondition", default)]
    pub evo_condition: Option<String>,
    /// Can this species hatch from an egg?
    /// JavaScript: canHatch?: boolean
    #[serde(rename = "canHatch", default)]
    pub can_hatch: bool,
    /// Base species (for formes)
    /// JavaScript: baseSpecies?: string
    #[serde(default)]
    pub base_species: Option<String>,
    /// Base forme name
    /// JavaScript: baseForme?: string
    #[serde(rename = "baseForme", default)]
    pub base_forme: Option<String>,
    /// Forme name
    /// JavaScript: forme?: string
    #[serde(default)]
    pub forme: Option<String>,
    /// Changes from (base forme this changes from)
    /// JavaScript: changesFrom?: string
    #[serde(rename = "changesFrom", default)]
    pub changes_from: Option<String>,
    /// Other formes
    /// JavaScript: otherFormes?: string[]
    #[serde(default)]
    pub other_formes: Vec<String>,
    /// Cosmetic formes
    /// JavaScript: cosmeticFormes?: string[]
    #[serde(default)]
    pub cosmetic_formes: Vec<String>,
    /// Is this a cosmetic forme?
    /// JavaScript: isCosmeticForme?: boolean
    #[serde(default)]
    pub is_cosmetic_forme: bool,
    /// Generation introduced
    /// JavaScript: gen?: number
    #[serde(default)]
    pub gen: Option<u8>,
    /// Tags
    /// JavaScript: tags?: string[]
    #[serde(default)]
    pub tags: Vec<String>,
    /// Egg groups
    /// JavaScript: eggGroups?: string[]
    #[serde(rename = "eggGroups", default)]
    pub egg_groups: Vec<String>,
    /// Battle-only forme
    /// JavaScript: battleOnly?: string | string[]
    /// TODO: Rust uses StringOrVec enum, JavaScript uses union type
    #[serde(rename = "battleOnly", default)]
    pub battle_only: StringOrVec,
    /// Forme order
    /// JavaScript: formeOrder?: string[]
    #[serde(rename = "formeOrder", default)]
    pub forme_order: Vec<String>,
    /// Required items for forme change
    /// JavaScript: requiredItems?: string[]
    #[serde(rename = "requiredItems", default)]
    pub required_items: Vec<String>,
    /// True if this Pokemon is a Mega Evolution
    /// JavaScript: isMega?: boolean
    #[serde(rename = "isMega", default)]
    pub is_mega: bool,
    /// Required move to use this forme in-battle
    /// JavaScript: requiredMove?: string
    #[serde(rename = "requiredMove", default)]
    pub required_move: Option<String>,
    /// Required ability to use this forme in-battle
    /// JavaScript: requiredAbility?: string
    #[serde(rename = "requiredAbility", default)]
    pub required_ability: Option<String>,
    /// Can Gigantamax (G-Max move name)
    /// JavaScript: canGigantamax?: string
    #[serde(rename = "canGigantamax", default)]
    pub can_gigantamax: Option<String>,
    // Format data fields
    /// Tier in formats
    /// JavaScript: tier?: string
    #[serde(default)]
    pub tier: Option<String>,
    /// Doubles tier
    /// JavaScript: doublesTier?: string
    #[serde(default)]
    pub doubles_tier: Option<String>,
    /// National Dex tier
    /// JavaScript: natDexTier?: string
    #[serde(rename = "natDexTier", default)]
    pub nat_dex_tier: Option<String>,
    /// Nonstandard status
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(default)]
    pub is_nonstandard: Option<String>,
    /// Unreleased hidden ability
    /// JavaScript: unreleasedHidden?: boolean | 'Past'
    #[serde(rename = "unreleasedHidden", default)]
    pub unreleased_hidden: Option<StringOrBool>,
    /// Does this species exist?
    /// JavaScript: exists: boolean
    #[serde(default = "default_true")]
    pub exists: bool,
}

/// Move secondary effect data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: MoveSecondary/SecondaryEffect (sim/dex-moves.ts)
/// 9 fields in JavaScript (matching full SecondaryEffect)
pub struct MoveSecondary {
    /// Chance of effect occurring (percentage)
    /// JavaScript: chance?: number
    #[serde(default)]
    pub chance: Option<i32>,
    /// Status to inflict
    /// JavaScript: status?: string
    #[serde(default)]
    pub status: Option<String>,
    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<HashMap<String, i32>>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status_secondary: Option<String>,
    /// Side condition to apply
    /// JavaScript: sideCondition?: string
    #[serde(rename = "sideCondition", default)]
    pub side_condition: Option<String>,
    /// Slot condition to apply
    /// JavaScript: slotCondition?: string
    #[serde(rename = "slotCondition", default)]
    pub slot_condition: Option<String>,
    /// Pseudo weather to apply
    /// JavaScript: pseudoWeather?: string
    #[serde(rename = "pseudoWeather", default)]
    pub pseudo_weather: Option<String>,
    /// Terrain to apply
    /// JavaScript: terrain?: string
    #[serde(default)]
    pub terrain: Option<String>,
    /// Weather to apply
    /// JavaScript: weather?: string
    #[serde(default)]
    pub weather: Option<String>,
    /// Self-targeted secondary effect (applied to user instead of target)
    /// JavaScript: self?: SecondaryEffect
    #[serde(rename = "self", default)]
    pub self_secondary: Option<Box<MoveSecondary>>,
}

/// Type of condition
/// JavaScript equivalent: EffectType for conditions (sim/dex-conditions.ts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ConditionType {
    /// Non-volatile status (burn, paralysis, poison, sleep, freeze)
    Status,
    /// Volatile status (confusion, taunt, encore, etc.)
    #[default]
    Volatile,
    /// Side condition (stealth rock, spikes, reflect, etc.)
    SideCondition,
    /// Slot condition (Wish, Healing Wish)
    SlotCondition,
    /// Weather (rain, sun, sand, hail)
    Weather,
    /// Terrain (electric, grassy, psychic, misty)
    Terrain,
    /// Pseudo-weather (Trick Room, Magic Room, etc.)
    PseudoWeather,
}

/// Condition data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: ConditionData (sim/dex-conditions.ts)
/// Fields match JavaScript data fields (not callback implementations)
pub struct ConditionData {
    /// Condition name
    /// JavaScript: name?: string
    /// Note: Optional because embedded conditions in moves don't always have names
    #[serde(default)]
    pub name: Option<String>,
    /// Effect type
    /// JavaScript: effectType?: 'Status' | 'Weather' | 'Terrain' | 'SideCondition' | 'SlotCondition' | 'PseudoWeather'
    #[serde(rename = "effectType", default)]
    pub effect_type: Option<String>,
    /// Duration in turns (None = indefinite or until cured)
    /// JavaScript: duration?: number
    #[serde(default)]
    pub duration: Option<i32>,
    /// Cannot be passed by Baton Pass
    /// JavaScript: noCopy?: boolean
    #[serde(rename = "noCopy", default)]
    pub no_copy: bool,
    /// Counter maximum (for stall, etc.)
    /// JavaScript: counterMax?: number
    #[serde(rename = "counterMax", default)]
    pub counter_max: Option<i32>,
    /// Affects fainted Pokemon
    /// JavaScript: affectsFainted?: boolean
    #[serde(rename = "affectsFainted", default)]
    pub affects_fainted: bool,
    /// Extra fields (like onResidualOrder, callback flags, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ConditionData {
    /// Get the condition type
    pub fn condition_type(&self) -> ConditionType {
        match self.effect_type.as_deref() {
            Some("Status") => ConditionType::Status,
            Some("Weather") => ConditionType::Weather,
            Some("Terrain") => ConditionType::Terrain,
            Some("SideCondition") => ConditionType::SideCondition,
            Some("SlotCondition") => ConditionType::SlotCondition,
            Some("PseudoWeather") => ConditionType::PseudoWeather,
            _ => ConditionType::Volatile,
        }
    }

    /// Get effect type string
    /// JavaScript equivalent: condition.effectType
    /// Returns the effectType as a string: 'Status', 'Weather', 'Terrain', or 'Condition'
    pub fn effect_type(&self) -> &str {
        match self.effect_type.as_deref() {
            Some("Status") => "Status",
            Some("Weather") => "Weather",
            Some("Terrain") => "Terrain",
            Some("SideCondition") => "Condition",
            Some("SlotCondition") => "Condition",
            Some("PseudoWeather") => "Condition",
            Some(other) => other,
            None => "Condition",
        }
    }

    /// Can be passed by Baton Pass (inverse of noCopy)
    pub fn baton_passable(&self) -> bool {
        !self.no_copy
    }
}

/// Move data from the Dex
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: MoveData (sim/dex-moves.ts)
/// 49 fields in Rust (matching JavaScript fields)
pub struct MoveData {
    /// Move number in the Dex
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Move name
    /// JavaScript: name: string
    pub name: String,
    /// Move ID
    /// JavaScript: id: ID
    #[serde(default)]
    pub id: ID, // Move ID (computed from name or provided)
    /// Move type
    /// JavaScript: type: string
    #[serde(rename = "type")]
    pub move_type: String,
    /// Move category (Physical/Special/Status)
    /// JavaScript: category: string
    pub category: String,
    /// Base power
    /// JavaScript: basePower: number
    #[serde(default)]
    pub base_power: i32,
    /// Accuracy (number or true for always hits)
    /// JavaScript: accuracy: true | number
    /// TODO: Rust uses Accuracy enum, JavaScript uses union type
    #[serde(default, deserialize_with = "deserialize_accuracy")]
    pub accuracy: Accuracy,
    /// Power Points
    /// JavaScript: pp: number
    #[serde(default)]
    pub pp: i32,
    /// Move priority (-7 to +7)
    /// JavaScript: priority: number
    #[serde(default)]
    pub priority: i8,
    /// Target type
    /// JavaScript: target: string
    #[serde(default)]
    pub target: String,
    /// Critical hit ratio
    /// JavaScript: critRatio: number
    #[serde(rename = "critRatio", default = "default_crit_ratio")]
    pub crit_ratio: i32,
    /// Secondary effect
    /// JavaScript: secondary?: SecondaryEffect | null
    #[serde(default)]
    pub secondary: Option<MoveSecondary>,
    /// Multiple secondary effects
    /// JavaScript: secondaries?: SecondaryEffect[] | null
    #[serde(default)]
    pub secondaries: Option<Vec<MoveSecondary>>,
    /// Self-targeting effect
    /// JavaScript: self?: SecondaryEffect | null
    #[serde(default, rename = "self")]
    pub self_effect: Option<MoveSecondary>,
    /// Move flags
    /// JavaScript: flags: MoveFlags
    #[serde(default)]
    pub flags: HashMap<String, i32>,
    /// Stat boosts
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<HashMap<String, i32>>,
    /// Status condition to inflict
    /// JavaScript: status?: string
    #[serde(default)]
    pub status: Option<String>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
    /// Condition data for moves that create conditions
    /// JavaScript: condition?: ConditionData
    #[serde(default)]
    pub condition: Option<ConditionData>,
    /// HP drain fraction [numerator, denominator]
    /// JavaScript: drain?: [number, number]
    #[serde(default)]
    pub drain: Option<(i32, i32)>,
    /// Recoil damage fraction [numerator, denominator]
    /// JavaScript: recoil?: [number, number]
    #[serde(default)]
    pub recoil: Option<(i32, i32)>,
    /// Healing fraction [numerator, denominator]
    /// JavaScript: heal?: [number, number]
    #[serde(default)]
    pub heal: Option<(i32, i32)>,
    /// Multi-hit specification
    /// JavaScript: multihit?: number | [number, number]
    /// TODO: Rust uses Multihit enum, JavaScript uses union type
    #[serde(default)]
    pub multihit: Option<Multihit>,
    /// Z-Move identifier
    /// JavaScript: isZ?: boolean | IDEntry
    /// TODO: Rust uses Option<String>, JavaScript uses boolean | IDEntry union
    #[serde(rename = "isZ", default)]
    pub is_z: Option<String>,
    /// Max Move identifier
    /// JavaScript: isMax?: boolean | string
    /// TODO: Rust uses IsMax enum, JavaScript uses boolean | string union
    #[serde(rename = "isMax", default, deserialize_with = "deserialize_is_max")]
    pub is_max: Option<IsMax>,
    /// OHKO type
    /// JavaScript: ohko?: boolean | 'Ice'
    /// TODO: Rust uses Ohko enum, JavaScript uses boolean | string union
    #[serde(default, deserialize_with = "deserialize_ohko")]
    pub ohko: Option<Ohko>,
    /// Self-destruct type
    /// JavaScript: selfdestruct?: 'always' | 'ifHit' | boolean
    /// TODO: Rust uses Option<String>, JavaScript uses union type
    #[serde(default)]
    pub selfdestruct: Option<String>,
    /// Tracks target location
    /// JavaScript: tracksTarget?: boolean
    #[serde(rename = "tracksTarget", default)]
    pub tracks_target: Option<bool>,
    /// Smart target selection
    /// JavaScript: smartTarget?: boolean
    #[serde(rename = "smartTarget", default)]
    pub smart_target: Option<bool>,
    /// Base move (for moves that call other moves)
    /// JavaScript: baseMove?: ID
    #[serde(rename = "baseMove", default)]
    pub base_move: Option<ID>,
    /// Is Z or Max powered
    /// JavaScript: isZOrMaxPowered?: boolean
    #[serde(rename = "isZOrMaxPowered", default)]
    pub is_z_or_max_powered: bool,
    /// Will always crit
    /// JavaScript: willCrit?: boolean
    #[serde(rename = "willCrit", default)]
    pub will_crit: Option<bool>,
    /// Contest type (Beautiful, Cool, Cute, Clever, Tough)
    /// JavaScript: contestType?: string
    #[serde(rename = "contestType", default)]
    pub contest_type: Option<String>,
    /// Z-Move options (basePower, effect, boost)
    /// JavaScript: zMove?: { basePower?: number, effect?: string, boost?: SparseBoostsTable }
    #[serde(rename = "zMove", default)]
    pub z_move: Option<serde_json::Value>,
    /// Max Move options (basePower)
    /// JavaScript: maxMove?: { basePower?: number }
    #[serde(rename = "maxMove", default)]
    pub max_move: Option<serde_json::Value>,
    /// Calls another move (like Metronome, Sleep Talk)
    /// JavaScript: callsMove?: boolean
    #[serde(rename = "callsMove", default)]
    pub calls_move: bool,
    /// Can be used while asleep
    /// JavaScript: sleepUsable?: boolean
    #[serde(rename = "sleepUsable", default)]
    pub sleep_usable: bool,
    /// Weather condition set by this move
    /// JavaScript: weather?: string
    #[serde(default)]
    pub weather: Option<String>,
    /// Terrain condition set by this move
    /// JavaScript: terrain?: string
    #[serde(default)]
    pub terrain: Option<String>,
    /// Side condition set by this move (Stealth Rock, Reflect, etc.)
    /// JavaScript: sideCondition?: string
    #[serde(rename = "sideCondition", default)]
    pub side_condition: Option<String>,
    /// Slot condition set by this move (Future Sight, etc.)
    /// JavaScript: slotCondition?: string
    #[serde(rename = "slotCondition", default)]
    pub slot_condition: Option<String>,
    /// Self-switch flag (U-turn, Baton Pass, etc.)
    /// JavaScript: selfSwitch?: boolean | string
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses boolean | string union
    #[serde(rename = "selfSwitch", default)]
    pub self_switch: Option<serde_json::Value>,
    /// Ignore target's ability
    /// JavaScript: ignoreAbility?: boolean
    #[serde(rename = "ignoreAbility", default)]
    pub ignore_ability: bool,
    /// Ignore defensive stat changes
    /// JavaScript: ignoreDefensive?: boolean
    #[serde(rename = "ignoreDefensive", default)]
    pub ignore_defensive: bool,
    /// Ignore accuracy stat changes
    /// JavaScript: ignoreAccuracy?: boolean
    #[serde(rename = "ignoreAccuracy", default)]
    pub ignore_accuracy: bool,
    /// Ignore evasion stat changes
    /// JavaScript: ignoreEvasion?: boolean
    #[serde(rename = "ignoreEvasion", default)]
    pub ignore_evasion: bool,
    /// Ignore type immunities
    /// JavaScript: ignoreImmunity?: boolean | { [type: string]: boolean }
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses boolean | object union
    #[serde(rename = "ignoreImmunity", default)]
    pub ignore_immunity: Option<serde_json::Value>,
    /// Fixed damage amount (like "level" for Seismic Toss)
    /// JavaScript: damage?: number | string | boolean
    #[serde(default)]
    pub damage: Option<serde_json::Value>,
    /// Forces target to switch out
    /// JavaScript: forceSwitch?: boolean
    #[serde(rename = "forceSwitch", default)]
    pub force_switch: bool,
    /// Breaks through Protect/Detect
    /// JavaScript: breaksProtect?: boolean
    #[serde(rename = "breaksProtect", default)]
    pub breaks_protect: bool,
    /// Is this a stalling move (Protect, Detect, etc.)
    /// JavaScript: stallingMove?: boolean
    #[serde(rename = "stallingMove", default)]
    pub stalling_move: bool,
    /// Pseudo-weather condition (Gravity, Fairy Lock, etc.)
    /// JavaScript: pseudoWeather?: string
    #[serde(rename = "pseudoWeather", default)]
    pub pseudo_weather: Option<String>,
    /// Thaws frozen target
    /// JavaScript: thawsTarget?: boolean
    #[serde(rename = "thawsTarget", default)]
    pub thaws_target: bool,
    /// Has Sheer Force boost
    /// JavaScript: hasSheerForce?: boolean
    #[serde(rename = "hasSheerForce", default)]
    pub has_sheer_force: bool,
    /// Override offensive stat (use SpA for physical move, etc.)
    /// JavaScript: overrideOffensiveStat?: StatID
    #[serde(rename = "overrideOffensiveStat", default)]
    pub override_offensive_stat: Option<String>,
    /// Override defensive stat (target Def instead of SpD)
    /// JavaScript: overrideDefensiveStat?: StatID
    #[serde(rename = "overrideDefensiveStat", default)]
    pub override_defensive_stat: Option<String>,
    /// Override offensive Pokemon (for Foul Play)
    /// JavaScript: overrideOffensivePokemon?: 'target' | 'source'
    #[serde(rename = "overrideOffensivePokemon", default)]
    pub override_offensive_pokemon: Option<String>,
    /// Has crash damage on miss (High Jump Kick, etc.)
    /// JavaScript: hasCrashDamage?: boolean
    #[serde(rename = "hasCrashDamage", default)]
    pub has_crash_damage: bool,
    /// Mind Blown recoil flag
    /// JavaScript: mindBlownRecoil?: boolean
    #[serde(rename = "mindBlownRecoil", default)]
    pub mind_blown_recoil: bool,
    /// Struggle recoil flag
    /// JavaScript: struggleRecoil?: boolean
    #[serde(rename = "struggleRecoil", default)]
    pub struggle_recoil: bool,
    /// Steals target's stat boosts (Spectral Thief)
    /// JavaScript: stealsBoosts?: boolean
    #[serde(rename = "stealsBoosts", default)]
    pub steals_boosts: bool,
    /// Self boost effect (stat boosts to user)
    /// JavaScript: selfBoost?: { boosts?: SparseBoostsTable }
    #[serde(rename = "selfBoost", default)]
    pub self_boost: Option<serde_json::Value>,
    /// Multi-hit accuracy check per hit (Population Bomb)
    /// JavaScript: multiaccuracy?: boolean
    #[serde(rename = "multiaccuracy", default)]
    pub multiaccuracy: bool,
    /// No PP boosts allowed (Revival Blessing)
    /// JavaScript: noPPBoosts?: boolean
    #[serde(rename = "noPPBoosts", default)]
    pub no_pp_boosts: bool,
    /// Non-Ghost type target override (for Curse)
    /// JavaScript: nonGhostTarget?: string
    #[serde(rename = "nonGhostTarget", default)]
    pub non_ghost_target: Option<String>,
    /// On-damage priority for ordering damage calculations
    /// JavaScript: onDamagePriority?: number
    #[serde(rename = "onDamagePriority", default)]
    pub on_damage_priority: Option<i32>,
    /// Real move name (for Hidden Power variants)
    /// JavaScript: realMove?: string
    #[serde(rename = "realMove", default)]
    pub real_move: Option<String>,
    /// Nonstandard status (Past, Future, Unobtainable, etc.)
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(rename = "isNonstandard", default)]
    pub is_nonstandard: Option<String>,
    /// Extra fields for callback flags and other dynamic data
    /// Note: JavaScript has many callback methods that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl MoveData {
    /// Get effect type
    /// JavaScript equivalent: move.effectType (always 'Move')
    /// In JavaScript, Move class declares: readonly effectType: 'Move'
    pub fn effect_type(&self) -> &'static str {
        "Move"
    }
}

/// Accuracy can be a number or true (always hits)
/// Rust uses enum to represent JavaScript's number | true union type
/// JavaScript: accuracy: number | true
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
/// TODO: Rust uses enum to represent JavaScript's true | string union type
/// JavaScript: isMax?: true | string
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
/// TODO: Rust uses enum to represent JavaScript's true | string union type
/// JavaScript: ohko?: true | string
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
/// TODO: Rust uses enum to represent JavaScript's number | [number, number] union type
/// JavaScript: multihit?: number | [number, number]
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

/// StringOrBool can be a boolean or a string
/// Rust uses enum to represent JavaScript's boolean | string union type
/// JavaScript: field: boolean | string
#[derive(Debug, Clone, Default)]
pub enum StringOrBool {
    #[default]
    None,
    Bool(bool),
    String(String),
}

impl<'de> Deserialize<'de> for StringOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};

        struct StringOrBoolVisitor;

        impl<'de> Visitor<'de> for StringOrBoolVisitor {
            type Value = StringOrBool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or string")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrBool::Bool(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrBool::String(value.to_string()))
            }
        }

        deserializer.deserialize_any(StringOrBoolVisitor)
    }
}

impl Serialize for StringOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrBool::None => serializer.serialize_none(),
            StringOrBool::Bool(b) => serializer.serialize_bool(*b),
            StringOrBool::String(s) => serializer.serialize_str(s),
        }
    }
}

impl StringOrBool {
    /// Returns true if this is a boolean value set to true
    pub fn is_true(&self) -> bool {
        matches!(self, StringOrBool::Bool(true))
    }

    /// Returns true if this is Some value (either bool or string)
    pub fn is_some(&self) -> bool {
        !matches!(self, StringOrBool::None)
    }
}

/// StringOrVec can be a single string or an array of strings
/// TODO: Rust uses enum to represent JavaScript's string | string[] union type
/// JavaScript: field: string | string[]
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

impl<'de> serde::Deserialize<'de> for Accuracy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AccuracyVisitor;

        impl<'de> serde::de::Visitor<'de> for AccuracyVisitor {
            type Value = Accuracy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean (true) or an integer")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                if value {
                    Ok(Accuracy::AlwaysHits)
                } else {
                    Err(E::custom("accuracy can only be true (always hits) or a number"))
                }
            }

            fn visit_i32<E>(self, value: i32) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value as i32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value as i32))
            }
        }

        deserializer.deserialize_any(AccuracyVisitor)
    }
}


/// Ability data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: AbilityData (sim/dex-abilities.ts)
/// 20+ fields in JavaScript (many are callbacks)
pub struct AbilityData {
    /// Ability number
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Ability name
    /// JavaScript: name: string
    pub name: String,
    /// Full description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Short description
    /// JavaScript: shortDesc?: string
    #[serde(rename = "shortDesc", default)]
    pub short_desc: Option<String>,
    /// Competitive rating
    /// JavaScript: rating?: number
    #[serde(default)]
    pub rating: Option<f64>,
    /// Ability flags
    /// JavaScript: flags?: { [k: string]: 1 }
    #[serde(default)]
    pub flags: HashMap<String, i32>,
    /// Effect type
    /// JavaScript: effectType?: string
    #[serde(default)]
    pub effect_type: Option<String>,
    /// Nonstandard status (Past, Future, Unobtainable, etc.)
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(rename = "isNonstandard", default)]
    pub is_nonstandard: Option<String>,
    /// Whether this ability suppresses weather
    /// JavaScript: suppressWeather?: boolean
    #[serde(rename = "suppressWeather", default)]
    pub suppress_weather: Option<bool>,
    /// Extra fields (like onResidualOrder, onResidualSubOrder, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods (onStart, onEnd, etc.) that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl AbilityData {
    /// Get effect type
    /// JavaScript equivalent: ability.effectType (always 'Ability')
    /// In JavaScript, this is set in the Ability constructor: this.effectType = 'Ability'
    pub fn effect_type(&self) -> &'static str {
        "Ability"
    }
}

/// Item data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: ItemData (sim/dex-items.ts)
/// 30+ fields in JavaScript (many are callbacks)
pub struct ItemData {
    /// Item number
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Item name
    /// JavaScript: name: string
    pub name: String,
    /// Description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Is this a Choice item? (Choice Band, Scarf, Specs)
    /// JavaScript: isChoice?: boolean
    #[serde(rename = "isChoice", default)]
    pub is_choice: bool,
    /// Is this a Berry?
    /// JavaScript: isBerry?: boolean
    #[serde(rename = "isBerry", default)]
    pub is_berry: bool,
    /// Is this a Gem? (Fire Gem, etc.)
    /// JavaScript: isGem?: boolean
    #[serde(rename = "isGem", default)]
    pub is_gem: bool,
    /// Fling data
    /// JavaScript: fling?: FlingData
    #[serde(default)]
    pub fling: Option<FlingData>,
    /// Natural Gift data (for berries)
    /// JavaScript: naturalGift?: { basePower: number, type: string }
    #[serde(rename = "naturalGift", default)]
    pub natural_gift: Option<serde_json::Value>,
    /// Type for Plate items (e.g., "Fire" for Flame Plate)
    /// JavaScript: onPlate?: string
    #[serde(rename = "onPlate", default)]
    pub on_plate: Option<String>,
    /// Type for Memory items (e.g., "Fire" for Fire Memory)
    /// JavaScript: onMemory?: string
    #[serde(rename = "onMemory", default)]
    pub on_memory: Option<String>,
    /// Z-Move compatibility
    /// JavaScript: zMove?: string | true | ZMoveOptions
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses union type
    #[serde(rename = "zMove", default)]
    pub z_move: Option<serde_json::Value>,
    /// Mega Evolution stone target (e.g., "Froslass-Mega")
    /// JavaScript: megaStone?: string
    /// TODO: Rust uses StringOrVec, JavaScript uses string
    #[serde(rename = "megaStone", default)]
    pub mega_stone: Option<StringOrVec>,
    /// Pokemon species that can use this mega stone (e.g., "Froslass")
    /// JavaScript: megaEvolves?: string
    /// TODO: Rust uses StringOrVec, JavaScript uses string
    #[serde(rename = "megaEvolves", default)]
    pub mega_evolves: Option<StringOrVec>,
    /// Species that can use this item
    /// JavaScript: itemUser?: string[]
    #[serde(rename = "itemUser", default)]
    pub item_user: Option<Vec<String>>,
    /// Stat boosts when item is used (e.g., for Cell Battery)
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<std::collections::HashMap<String, i32>>,
    /// Sprite number for UI display
    /// JavaScript: spritenum?: number
    #[serde(default)]
    pub spritenum: Option<i32>,
    /// Generation introduced
    /// JavaScript: gen?: number
    #[serde(default)]
    pub gen: Option<u8>,
    /// Is this a Pok&eacute; Ball?
    /// JavaScript: isPokeball?: boolean
    #[serde(rename = "isPokeball", default)]
    pub is_pokeball: bool,
    /// Does this item ignore the Klutz ability?
    /// JavaScript: ignoreKlutz?: boolean
    #[serde(rename = "ignoreKlutz", default)]
    pub ignore_klutz: bool,
    /// Nonstandard status (Past, Future, Unobtainable, etc.)
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(rename = "isNonstandard", default)]
    pub is_nonstandard: Option<String>,
    /// Extra fields (like onResidualOrder, onResidualSubOrder, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods (onStart, onEnd, etc.) that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ItemData {
    /// Get effect type
    /// JavaScript equivalent: item.effectType (always 'Item')
    /// In JavaScript, this is set in the Item constructor: this.effectType = 'Item'
    pub fn effect_type(&self) -> &'static str {
        "Item"
    }
}

/// Fling data for items that can be flung
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: FlingData (sim/dex-items.ts)
/// 4 fields in JavaScript
pub struct FlingData {
    /// Base power of Fling when using this item
    /// JavaScript: basePower: number
    #[serde(rename = "basePower", default)]
    pub base_power: i32,
    /// Effect when item is flung
    /// JavaScript: effect?: string
    #[serde(default)]
    pub effect: Option<String>,
    /// Status inflicted by Fling
    /// JavaScript: status?: string
    #[serde(default)]
    pub status: Option<String>,
    /// Volatile status inflicted by Fling
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
}

/// Type effectiveness data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: TypeData (sim/dex-data.ts)
/// ~4 fields in JavaScript
pub struct TypeData {
    /// Type matchup chart (opponent type -> effectiveness)
    /// JavaScript: damageTaken: { [attackingType: string]: number }
    #[serde(rename = "damageTaken")]
    pub damage_taken: HashMap<String, u8>,
}

/// Ruleset data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: RulesetData (sim/dex-formats.ts)
/// 2 fields in JavaScript
pub struct RulesetData {
    /// Ruleset name
    /// JavaScript: name: string
    pub name: String,
    /// Mod ID (optional)
    /// JavaScript: mod?: string
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
}

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: NatureData (sim/dex-data.ts)
/// 3 fields in JavaScript
pub struct NatureData {
    /// Nature name
    /// JavaScript: name: string
    pub name: String,
    /// Stat that gets a 1.1x boost
    /// JavaScript: plus?: StatIDExceptHP
    #[serde(default)]
    pub plus: Option<String>,
    /// Stat that gets a 0.9x penalty
    /// JavaScript: minus?: StatIDExceptHP
    #[serde(default)]
    pub minus: Option<String>,
}

/// Event data for learnsets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: EventInfo (sim/dex-species.ts)
/// 4 fields in JavaScript
pub struct EventData {
    /// Generation of the event
    /// JavaScript: generation?: number
    #[serde(default)]
    pub generation: Option<u8>,
    /// Level of the event Pokemon
    /// JavaScript: level?: number
    #[serde(default)]
    pub level: Option<u8>,
    /// Moves the event Pokemon knows
    /// JavaScript: moves?: string[]
    #[serde(default)]
    pub moves: Vec<String>,
    /// Source/description of the event
    /// JavaScript: source?: string
    #[serde(default)]
    pub source: Option<String>,
}

/// Learnset data for a single species
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: LearnsetData (sim/dex-species.ts)
/// Fields: learnset, eventData, eventOnly, encounters, exists
pub struct LearnsetData {
    /// Map from move ID to learn methods (e.g., "9M", "8L15", "7E")
    /// JavaScript: learnset: { [moveid: string]: string[] }
    #[serde(default)]
    pub learnset: HashMap<String, Vec<String>>,
    /// Event-only moves
    /// JavaScript: eventData?: EventInfo[]
    #[serde(default)]
    pub event_data: Option<Vec<EventData>>,
    /// Is this Pokemon event-only?
    /// JavaScript: eventOnly?: boolean
    #[serde(default)]
    pub event_only: Option<bool>,
}

/// Format data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: Format (sim/dex-formats.ts)
/// ~20 fields in JavaScript
pub struct FormatData {
    /// Format name
    /// JavaScript: name: string
    pub name: String,
    /// Mod ID
    /// JavaScript: mod?: string
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
    /// Team setting
    /// JavaScript: team?: string
    #[serde(default)]
    pub team: Option<String>,
    /// Game type
    /// JavaScript: gameType?: string
    #[serde(default)]
    pub game_type: Option<String>,
    /// Description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Debug mode
    /// JavaScript: debug: boolean
    #[serde(default)]
    pub debug: bool,
    /// Rated format
    /// JavaScript: rated?: boolean | string
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses boolean | string union
    #[serde(default)]
    pub rated: Option<serde_json::Value>, // can be bool or string
    /// Search show
    /// JavaScript: searchShow?: boolean
    #[serde(default)]
    pub search_show: Option<bool>,
    /// Challenge show
    /// JavaScript: challengeShow?: boolean
    #[serde(default)]
    pub challenge_show: Option<bool>,
    /// Tournament show
    /// JavaScript: tournamentShow?: boolean
    #[serde(default)]
    pub tournament_show: Option<bool>,
    /// Best of default
    /// JavaScript: bestOfDefault?: boolean
    #[serde(default)]
    pub best_of_default: Option<bool>,
    /// Ruleset
    /// JavaScript: ruleset: string[]
    #[serde(default)]
    pub ruleset: Vec<String>,
    /// Banlist
    /// JavaScript: banlist: string[]
    #[serde(default)]
    pub banlist: Vec<String>,
    /// Restricted list
    /// JavaScript: restricted: string[]
    #[serde(default)]
    pub restricted: Vec<String>,
    /// Unban list
    /// JavaScript: unbanlist: string[]
    #[serde(default)]
    pub unbanlist: Vec<String>,
    /// Custom rules
    /// JavaScript: customRules?: string[]
    #[serde(default)]
    pub custom_rules: Option<Vec<String>>,
}

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

/// Embedded data for compile-time inclusion
pub mod embedded {
    pub const SPECIES_JSON: &str = include_str!("../data/species.json");
    pub const MOVES_JSON: &str = include_str!("../data/moves.json");
    pub const ABILITIES_JSON: &str = include_str!("../data/abilities.json");
    pub const ITEMS_JSON: &str = include_str!("../data/items.json");
    pub const CONDITIONS_JSON: &str = include_str!("../data/conditions.json");
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
