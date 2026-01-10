//! Pokemon species data from the Dex

use serde::{Deserialize, Serialize};

use super::default_true::default_true;
use super::{AbilitySlots, BaseStatsData, GenderRatio, StringOrBool, StringOrVec};

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
