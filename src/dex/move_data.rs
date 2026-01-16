//! Move data from the Dex

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dex_data::ID;

use super::deserialize_accuracy::deserialize_accuracy;
use super::deserialize_damage::deserialize_damage;
use super::deserialize_ignore_immunity::deserialize_ignore_immunity;
use super::deserialize_is_max::deserialize_is_max;
use super::deserialize_move_flags::deserialize_move_flags;
use super::deserialize_ohko::deserialize_ohko;
use super::deserialize_self_boost::deserialize_self_boost;
use super::deserialize_self_switch::deserialize_self_switch;
use super::default_crit_ratio::default_crit_ratio;
use super::{Accuracy, ConditionData, IsMax, MoveSecondary, Multihit, Ohko};

/// Move data from the Dex
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    #[serde(default, deserialize_with = "deserialize_move_flags")]
    pub flags: crate::battle_actions::MoveFlags,
    /// Stat boosts
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<crate::dex_data::BoostsTable>,
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
    #[serde(rename = "multihit", default)]
    pub multi_hit: Option<Multihit>,
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
    #[serde(rename = "selfdestruct", default)]
    pub self_destruct: Option<String>,
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
    pub z_move: Option<crate::battle_actions::ZMoveData>,
    /// Max Move options (basePower)
    /// JavaScript: maxMove?: { basePower?: number }
    #[serde(rename = "maxMove", default)]
    pub max_move: Option<crate::battle_actions::MaxMoveData>,
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
    #[serde(rename = "selfSwitch", default, deserialize_with = "deserialize_self_switch")]
    pub self_switch: Option<String>,
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
    #[serde(rename = "ignoreImmunity", default, deserialize_with = "deserialize_ignore_immunity")]
    pub ignore_immunity: Option<crate::battle_actions::IgnoreImmunity>,
    /// Fixed damage amount (like "level" for Seismic Toss)
    /// JavaScript: damage?: number | string | boolean
    #[serde(default, deserialize_with = "deserialize_damage")]
    pub damage: Option<crate::battle_actions::Damage>,
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
    pub mindblown_recoil: bool,
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
    #[serde(rename = "selfBoost", default, deserialize_with = "deserialize_self_boost")]
    pub self_boost: Option<crate::dex_data::BoostsTable>,
    /// Multi-hit accuracy check per hit (Population Bomb)
    /// JavaScript: multiaccuracy?: boolean
    #[serde(rename = "multiaccuracy", default)]
    pub multi_accuracy: bool,
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
