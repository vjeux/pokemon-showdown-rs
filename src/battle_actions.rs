//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This is a 1:1 port of sim/battle-actions.ts
//! Handles all battle actions: moves, switches, damage calculation, etc.

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::dex::{Dex};
use crate::dex_data::{BoostsTable, ID};

/// Choosable target types for moves

// Function modules

// Function modules
mod new;
mod get_boost_modifier;
mod calc_recoil_damage;
mod get_confusion_damage;
mod target_type_choices;
mod can_ultra_burst;
mod can_terastallize;
mod run_z_power;
mod use_move;
mod use_move_inner;
mod switch_in;
mod drag_in;
mod run_switch;
mod get_damage;
mod get_spread_damage;
mod modify_damage;
mod run_move_effects;
mod spread_move_hit;
mod try_spread_move_hit;
mod hit_step_accuracy;
mod run_move;
pub use use_move::use_move;
pub use use_move_inner::use_move_inner;
pub use switch_in::switch_in;
pub use drag_in::drag_in;
pub use run_switch::run_switch;
pub use get_damage::get_damage;
pub use get_spread_damage::get_spread_damage;
pub use modify_damage::modify_damage;
pub use run_move_effects::run_move_effects;
pub use spread_move_hit::spread_move_hit;
pub use try_spread_move_hit::try_spread_move_hit;
pub use hit_step_accuracy::hit_step_accuracy;
pub use run_move::run_move;

pub static CHOOSABLE_TARGETS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("normal");
    set.insert("any");
    set.insert("adjacentAlly");
    set.insert("adjacentAllyOrSelf");
    set.insert("adjacentFoe");
    set
});

/// Parameters for Z-move functions
pub struct ZMoveParams<'a> {
    pub move_name: &'a str,
    pub move_type: &'a str,
    pub move_category: &'a str,
    pub z_move_base_power: Option<i32>,
    pub item_z_move: Option<&'a str>,
    pub item_z_move_from: Option<&'a str>,
    pub item_z_move_type: Option<&'a str>,
    pub z_move_used: bool,
}

/// Parameters for can_z_move function
pub struct CanZMoveParams<'a> {
    pub z_move_used: bool,
    pub is_transformed: bool,
    pub species_is_mega: bool,
    pub species_is_primal: bool,
    pub species_forme: &'a str,
    pub item_z_move: bool,
    pub item_user: Option<&'a [String]>,
    pub species_name: &'a str,
}

/// Parameters for get_damage function
pub struct DamageCalcParams<'a> {
    pub attacker_level: i32,
    pub attacker_attack: i32,
    pub defender_defense: i32,
    pub base_power: i32,
    pub stab_modifier: f64,
    pub type_effectiveness: f64,
    pub is_crit: bool,
    pub random_factor: i32,
    pub other_modifiers: &'a [f64],
}

/// Damage calculation result
#[derive(Debug, Clone)]
pub enum DamageResult {
    /// Actual damage dealt
    Damage(i32),
    /// Target is immune
    Immune,
    /// Move missed
    Miss,
    /// Move failed for some other reason
    Failed,
    /// No damage (status move or 0 base power)
    NoDamage,
}

/// Move hit data for tracking crits, effectiveness, etc.
/// Equivalent to MoveHitData in battle-actions.ts
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: MoveHitData (sim/dex-moves.ts)
/// 3 fields in JavaScript
pub struct MoveHitData {
    /// Critical hit flag
    /// JavaScript: crit: boolean
    pub crit: bool,
    /// Type effectiveness modifier
    /// JavaScript: typeMod: number
    pub type_mod: i32,
    // TODO: DELETE - Not in JavaScript MoveHitData
    /// Damage dealt (Rust-specific tracking)
    pub damage: i32,
    /// Z-Move broke through protect
    /// JavaScript: zBrokeProtect: boolean
    pub z_broke_protect: bool,
}

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: ActiveMove (sim/dex-moves.ts)
/// 35+ fields in JavaScript (plus inherited from MutableMove which is BasicEffect & MoveData & HitEffect)
/// JavaScript equivalent: ActiveMove (sim/global-types.ts)
pub struct ActiveMove {
    /// Effect type constant (always "Move")
    /// JavaScript: readonly effectType: 'Move'
    // This is implicit in Rust - all ActiveMove instances are moves

    // =========================================================================
    // From BasicEffect (inherited via MutableMove)
    // =========================================================================
    pub id: ID,
    pub name: String,
    /// Full name with effect type
    /// JavaScript: fullname: string
    pub fullname: String,
    /// Move number in the Dex
    /// JavaScript: num: number
    pub num: i32,
    /// Does this move exist?
    /// JavaScript: exists: boolean
    pub exists: bool,
    /// Generation this move was introduced
    /// JavaScript: gen: number
    pub gen: u8,
    /// Short description
    /// JavaScript: shortDesc: string
    pub short_desc: String,
    /// Full description
    /// JavaScript: desc: string
    pub desc: String,
    /// Nonstandard status (Past, Future, etc.)
    /// JavaScript: isNonstandard: Nonstandard | null
    pub is_nonstandard: Option<String>,
    /// Effect duration
    /// JavaScript: duration?: number
    pub duration: Option<i32>,
    /// Cannot be copied by Baton Pass
    /// JavaScript: noCopy: boolean
    pub no_copy: bool,
    /// Affects fainted Pokemon
    /// JavaScript: affectsFainted: boolean
    pub affects_fainted: bool,
    /// Source effect that created this move
    /// JavaScript: sourceEffect: string
    pub source_effect_name: Option<String>,

    // =========================================================================
    // From MoveData (inherited via MutableMove)
    // =========================================================================
    /// Condition data for moves that create conditions
    /// JavaScript: condition?: ConditionData
    pub condition: Option<String>, // TODO: Should be ConditionData type when available
    pub base_power: i32,
    /// Accuracy value
    /// JavaScript: accuracy: true | number
    /// TODO: Rust uses i32, cannot represent 'true' variant (true = always hits)
    pub accuracy: i32,
    /// Power Points
    /// JavaScript: pp: number
    pub pp: u8,
    pub category: String,
    pub move_type: String,
    pub priority: i8,
    pub target: String,
    pub flags: MoveFlags,
    /// Real move ID (for called/transformed moves)
    /// JavaScript: realMove?: string
    pub real_move: Option<String>,
    /// Fixed damage value
    /// JavaScript: damage?: number | 'level' | false | null
    /// TODO: Rust uses Option<i32>, cannot represent 'level' or false variants
    pub damage: Option<i32>,
    /// Contest type
    /// JavaScript: contestType?: string
    pub contest_type: Option<String>,
    /// No PP boosts allowed
    /// JavaScript: noPPBoosts?: boolean
    pub no_pp_boosts: Option<bool>,
    /// Is this a Z-move?
    /// JavaScript: isZ?: boolean | IDEntry
    /// TODO: Rust uses bool, cannot represent IDEntry variant (specific Z-crystal)
    pub is_z: bool,
    /// Z-move data
    /// JavaScript: zMove?: { basePower?: number, effect?: IDEntry, boost?: SparseBoostsTable }
    pub z_move: Option<ZMoveData>,
    /// Is this a Max move?
    /// JavaScript: isMax?: boolean | string
    /// TODO: Rust uses bool, cannot represent string variant (Gmax move name)
    pub is_max: bool,
    /// Max move data
    /// JavaScript: maxMove?: { basePower: number }
    pub max_move: Option<MaxMoveData>,
    /// OHKO type
    /// JavaScript: ohko?: boolean | 'Ice'
    pub ohko: Option<String>,
    /// Thaws the target
    /// JavaScript: thawsTarget?: boolean
    pub thaws_target: Option<bool>,
    /// Healing fraction
    /// JavaScript: heal?: number[] | null
    pub heal: Option<(i32, i32)>,
    /// Draining fraction
    /// JavaScript: drain?: [number, number]
    pub drain: Option<(i32, i32)>,
    pub force_switch: bool,
    /// Self-switch effect
    /// JavaScript: selfSwitch?: 'copyvolatile' | 'shedtail' | boolean
    /// TODO: Rust uses Option<String>, cannot fully represent boolean variant
    pub self_switch: Option<String>,
    pub self_boost: Option<BoostsTable>,
    /// Self-destruct type
    /// JavaScript: selfdestruct?: 'always' | 'ifHit' | boolean
    /// TODO: Rust uses Option<String>, cannot fully represent boolean variant
    pub self_destruct: Option<String>,
    pub breaks_protect: bool,
    pub recoil: Option<(i32, i32)>,
    pub mindblown_recoil: bool,
    pub steals_boosts: bool,
    pub struggle_recoil: bool,
    /// Single secondary effect
    /// JavaScript: secondary?: SecondaryEffect | null
    pub secondary: Option<SecondaryEffect>,
    /// Multiple secondary effects
    /// JavaScript: secondaries?: SecondaryEffect[] | null
    pub secondaries: Vec<SecondaryEffect>,
    /// Self-targeting effect
    /// JavaScript: self?: SecondaryEffect | null
    pub self_effect: Option<SelfEffect>,
    pub has_sheer_force: bool,
    pub always_hit: bool,
    /// Base move type before type-changing effects
    /// JavaScript: baseMoveType?: string
    pub base_move_type: Option<String>,
    /// Base power modifier
    /// JavaScript: basePowerModifier?: number
    pub base_power_modifier: Option<f64>,
    /// Critical hit modifier
    /// JavaScript: critModifier?: number
    pub crit_modifier: Option<f64>,
    pub crit_ratio: i32,
    /// Override which Pokemon's offensive stat to use
    /// JavaScript: overrideOffensivePokemon?: 'target' | 'source'
    pub override_offensive_pokemon: Option<String>,
    pub override_offensive_stat: Option<String>,
    /// Override which Pokemon's defensive stat to use
    /// JavaScript: overrideDefensivePokemon?: 'target' | 'source'
    pub override_defensive_pokemon: Option<String>,
    /// Override which defensive stat to use
    /// JavaScript: overrideDefensiveStat?: StatIDExceptHP
    pub override_defensive_stat: Option<String>,
    pub force_stab: bool,
    pub ignore_ability: bool,
    pub ignore_accuracy: bool,
    pub ignore_evasion: bool,
    /// Ignore positive evasion boosts
    /// JavaScript: ignorePositiveEvasion?: boolean
    pub ignore_positive_evasion: Option<bool>,
    pub ignore_immunity: Option<bool>,
    pub ignore_defensive: bool,
    pub ignore_offensive: bool,
    pub ignore_negative_offensive: bool,
    pub ignore_positive_defensive: bool,
    pub infiltrates: bool,
    pub will_crit: Option<bool>,
    pub multi_accuracy: bool,
    pub multi_hit: Option<i32>,
    pub multi_hit_type: Option<String>,
    /// No damage variance (no random factor)
    /// JavaScript: noDamageVariance?: boolean
    pub no_damage_variance: Option<bool>,
    /// Non-ghost target type (for Curse)
    /// JavaScript: nonGhostTarget?: MoveTarget
    /// TODO: Rust uses Option<String>, should reference MoveTarget type
    pub non_ghost_target: Option<String>,
    /// Spread move damage modifier
    /// JavaScript: spreadModifier?: number
    pub spread_modifier: Option<f64>,
    pub sleep_usable: bool,
    pub smart_target: Option<bool>,
    pub tracks_target: bool,
    /// Calls another move
    /// JavaScript: callsMove?: boolean
    pub calls_move: Option<bool>,
    /// Has crash damage on miss (High Jump Kick, etc.)
    /// JavaScript: hasCrashDamage?: boolean
    pub has_crash_damage: Option<bool>,
    /// Is this confusion self-hit?
    /// JavaScript: isConfusionSelfHit?: boolean
    pub is_confusion_self_hit: Option<bool>,
    /// Is this a stalling move (Protect, etc.)?
    /// JavaScript: stallingMove?: boolean
    pub stalling_move: Option<bool>,
    /// Base move ID (for moves that call other moves)
    /// JavaScript: baseMove?: ID
    pub base_move: Option<ID>,

    // =========================================================================
    // From HitEffect (inherited via MoveData)
    // =========================================================================
    pub boosts: Option<BoostsTable>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub side_condition: Option<String>,
    pub slot_condition: Option<String>,
    pub pseudo_weather: Option<String>,
    pub terrain: Option<String>,
    pub weather: Option<String>,

    // =========================================================================
    // ActiveMove-specific fields
    // =========================================================================
    pub hit: i32,
    /// Total damage dealt
    /// JavaScript: totalDamage: number | false
    /// TODO: Rust uses i32, cannot represent false variant
    pub total_damage: i32,
    /// Move hit data for tracking crit, type effectiveness, etc.
    /// JavaScript: moveHitData?: MoveHitData
    pub move_hit_data: Option<crate::pokemon::MoveHitData>,
    pub spread_hit: bool,
    /// Last hit of a multi-hit move
    /// JavaScript: lastHit?: boolean
    pub last_hit: Option<bool>,
    pub is_external: bool,
    pub is_z_or_max_powered: bool,
    pub prankster_boosted: bool,
    pub has_bounced: bool,
    /// Source effect that triggered this move (Dancer, Instruct, etc.)
    /// JavaScript: sourceEffect?: ID
    pub source_effect: Option<ID>,
    /// Aura Break ability is active
    /// JavaScript: hasAuraBreak?: boolean
    pub has_aura_break: Option<bool>,
    /// Pokemon with aura ability boosting this move
    /// JavaScript: auraBooster?: Pokemon
    pub aura_booster: Option<(usize, usize)>,
    /// Move caused crash damage (High Jump Kick, Jump Kick)
    /// JavaScript: causedCrashDamage?: boolean
    pub caused_crash_damage: Option<bool>,
    pub self_dropped: bool,
    pub stellar_boosted: bool,
    /// Effect ID that changed this move's type (e.g., "pixilate", "aerilate")
    /// Used to boost power after type change
    /// JavaScript: typeChangerBoosted?: Effect
    /// TODO: Rust uses Option<ID>, JavaScript uses full Effect object
    pub type_changer_boosted: Option<ID>,
    /// Magnitude value for Magnitude move
    /// JavaScript: magnitude?: number
    pub magnitude: Option<i32>,
    /// Whether this move will cause a forme change (relicsong)
    pub will_change_forme: bool,
    /// Status roll result for secondary status effects
    /// JavaScript: statusRoll?: string
    pub status_roll: Option<String>,
    pub force_status: Option<String>,

    // Tablets of Ruin tracking
    /// Pokemon with Tablets of Ruin lowering this move's attack
    /// JavaScript: ruinedAtk?: Pokemon
    pub ruined_atk: Option<(usize, usize)>,
    /// Pokemon with Vessel of Ruin lowering this move's sp. attack
    /// JavaScript: ruinedSpA?: Pokemon
    pub ruined_spa: Option<(usize, usize)>,
    /// Pokemon with Sword of Ruin lowering target's defense
    /// JavaScript: ruinedDef?: Pokemon
    pub ruined_def: Option<(usize, usize)>,
    /// Pokemon with Beads of Ruin lowering target's sp. defense
    /// JavaScript: ruinedSpD?: Pokemon
    pub ruined_spd: Option<(usize, usize)>,

    // Allies for spread moves
    /// Allied Pokemon for spread move calculations
    /// JavaScript: allies?: Pokemon[]
    pub allies: Option<Vec<(usize, usize)>>,
    /// Ability affecting this move
    /// JavaScript: ability?: Ability
    // TODO: Change to Ability struct when available
    pub ability: Option<ID>,

    // Hit targets (populated during execution)
    /// Targets hit by this move
    /// JavaScript: hitTargets?: Pokemon[]
    pub hit_targets: Vec<(usize, usize)>,
}

/// Move flags
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: MoveFlags (sim/dex-moves.ts)
/// 37 fields in JavaScript
pub struct MoveFlags {
    pub allyanim: bool,
    pub bite: bool,
    pub bullet: bool,
    pub bypasssub: bool,
    pub cant_use_twice: bool,
    pub charge: bool,
    pub contact: bool,
    pub dance: bool,
    pub defrost: bool,
    pub distance: bool,
    pub failcopycat: bool,
    pub failencore: bool,
    pub failinstruct: bool,
    pub failmefirst: bool,
    pub failmimic: bool,
    pub future_move: bool,
    pub gravity: bool,
    pub heal: bool,
    pub metronome: bool,
    pub mirror: bool,
    pub mustpressure: bool,
    pub noassist: bool,
    pub nonsky: bool,
    pub noparentalbond: bool,
    pub nosketch: bool,
    pub nosleeptalk: bool,
    pub pledgecombo: bool,
    pub powder: bool,
    pub protect: bool,
    pub pulse: bool,
    pub punch: bool,
    pub recharge: bool,
    pub reflectable: bool,
    pub slicing: bool,
    pub snatch: bool,
    pub sound: bool,
    pub wind: bool,
}

/// Max move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: MaxMoveData (sim/dex-moves.ts)
/// 1 field in JavaScript
pub struct MaxMoveData {
    /// Base power of the Max Move
    /// JavaScript: basePower: number
    pub base_power: i32,
}

/// Z-move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: ZMoveData (sim/dex-moves.ts)
/// 3 fields in JavaScript
pub struct ZMoveData {
    /// Base power of the Z-Move
    /// JavaScript: basePower?: number
    pub base_power: Option<i32>,
    /// Stat boosts from Z-Move
    /// JavaScript: boost?: SparseBoostsTable
    pub boost: Option<BoostsTable>,
    /// Effect ID
    /// JavaScript: effect?: IDEntry
    pub effect: Option<String>,
}

/// Secondary effect data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: SecondaryEffect (sim/dex-moves.ts)
/// Extends HitEffect with additional fields
/// 13 fields total in JavaScript
pub struct SecondaryEffect {
    /// Chance of the effect occurring (percentage)
    pub chance: Option<i32>,

    // From HitEffect:
    /// onHit callback
    /// JavaScript: onHit?: MoveEventMethods['onHit']
    /// Note: Cannot store function reference in Rust, would need callback system
    // pub on_hit: Option<...>,

    pub boosts: Option<BoostsTable>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,

    /// Side condition to apply
    /// JavaScript: sideCondition?: string
    pub side_condition: Option<String>,

    /// Slot condition to apply
    /// JavaScript: slotCondition?: string
    pub slot_condition: Option<String>,

    /// Pseudo-weather to apply
    /// JavaScript: pseudoWeather?: string
    pub pseudo_weather: Option<String>,

    /// Terrain to apply
    /// JavaScript: terrain?: string
    pub terrain: Option<String>,

    /// Weather to apply
    /// JavaScript: weather?: string
    pub weather: Option<String>,

    // SecondaryEffect-specific fields:
    /// Ability data
    /// JavaScript: ability?: Ability
    /// Note: Cannot store full Ability, using ID instead
    pub ability: Option<ID>,

    /// King's Rock effect flag
    /// JavaScript: kingsrock?: boolean
    pub kingsrock: Option<bool>,

    /// Self-targeting effect
    /// JavaScript: self?: HitEffect
    /// Note: JavaScript uses HitEffect object, Rust uses simpler bool
    /// TODO: Should be Option<Box<HitEffect>> for full compatibility
    pub self_effect: bool,
}

/// Self effect data
/// JavaScript uses HitEffect for the `self` field in SecondaryEffect
/// Rust simplifies this with a dedicated struct
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: HitEffect (in SecondaryEffect.self field)
/// TODO: Missing fields from HitEffect: status, slotCondition, pseudoWeather, terrain, weather, onHit
pub struct SelfEffect {
    pub boosts: Option<BoostsTable>,
    pub chance: Option<i32>,
    pub side_condition: Option<String>,
    #[serde(rename = "volatileStatus")]
    pub volatile_status: Option<String>,
}

/// Z-Move request option
#[derive(Debug, Clone)]
pub struct ZMoveOption {
    pub move_name: String,
    pub target: String,
}

/// Damage value (can be number, false, or undefined-like None)
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(i32),
    Failed,
    Blocked, // HIT_SUBSTITUTE
}

/// Switch copy flag type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchCopyFlag {
    None,
    CopyVolatile,
    ShedTail,
}

/// Battle Actions struct - 1:1 port of BattleActions class
/// Note: In Rust, this struct needs a reference to battle state.
/// The actual methods that need battle access are implemented on Battle directly.
/// JavaScript equivalent: BattleActions (sim/battle-actions.ts)
/// 4 fields in JavaScript (battle, dex, MAX_MOVES, Z_MOVES)
/// JavaScript equivalent: BattleActions (sim/global-types.ts)
pub struct BattleActions<'a> {
    // TODO: Add battle reference when lifetime management is resolved
    // pub battle: &'a Battle,

    pub dex: &'a Dex,

    // TODO: DELETE - gen is not in JavaScript BattleActions
    // It's accessed via battle.gen instead
    pub gen: u8,
}

/// Max move names by type
/// JavaScript: readonly MAX_MOVES: { readonly [k: string]: string }
pub mod max_moves {
    pub const FLYING: &str = "Max Airstream";
    pub const DARK: &str = "Max Darkness";
    pub const FIRE: &str = "Max Flare";
    pub const BUG: &str = "Max Flutterby";
    pub const WATER: &str = "Max Geyser";
    pub const STATUS: &str = "Max Guard";
    pub const ICE: &str = "Max Hailstorm";
    pub const FIGHTING: &str = "Max Knuckle";
    pub const ELECTRIC: &str = "Max Lightning";
    pub const PSYCHIC: &str = "Max Mindstorm";
    pub const POISON: &str = "Max Ooze";
    pub const GRASS: &str = "Max Overgrowth";
    pub const GHOST: &str = "Max Phantasm";
    pub const GROUND: &str = "Max Quake";
    pub const ROCK: &str = "Max Rockfall";
    pub const FAIRY: &str = "Max Starfall";
    pub const STEEL: &str = "Max Steelspike";
    pub const NORMAL: &str = "Max Strike";
    pub const DRAGON: &str = "Max Wyrmwind";
}

/// Z-move names by type
/// JavaScript: readonly Z_MOVES: { readonly [k: string]: string }
pub mod z_moves {
    pub const POISON: &str = "Acid Downpour";
    pub const FIGHTING: &str = "All-Out Pummeling";
    pub const DARK: &str = "Black Hole Eclipse";
    pub const GRASS: &str = "Bloom Doom";
    pub const NORMAL: &str = "Breakneck Blitz";
    pub const ROCK: &str = "Continental Crush";
    pub const STEEL: &str = "Corkscrew Crash";
    pub const DRAGON: &str = "Devastating Drake";
    pub const ELECTRIC: &str = "Gigavolt Havoc";
    pub const WATER: &str = "Hydro Vortex";
    pub const FIRE: &str = "Inferno Overdrive";
    pub const GHOST: &str = "Never-Ending Nightmare";
    pub const BUG: &str = "Savage Spin-Out";
    pub const PSYCHIC: &str = "Shattered Psyche";
    pub const ICE: &str = "Subzero Slammer";
    pub const FLYING: &str = "Supersonic Skystrike";
    pub const GROUND: &str = "Tectonic Rage";
    pub const FAIRY: &str = "Twinkle Tackle";
}

impl<'a> BattleActions<'a> {
}

/// Result from after move secondary event
#[derive(Debug, Clone, Default)]
pub struct AfterMoveResult {
    pub self_switch: Option<String>,
    pub force_switch: bool,
}

/// Move effects to apply
#[derive(Debug, Clone, Default)]
pub struct MoveEffects {
    pub boosts: Option<BoostsTable>,
    pub heal: Option<(i32, i32)>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
}

/// Run move options for runMove
/// Equivalent to the options parameter in battle-actions.ts runMove()
#[derive(Debug, Clone, Default)]
pub struct RunMoveOptions {
    /// Source effect that caused this move
    pub source_effect: Option<ID>,
    /// Z-move override
    pub z_move: Option<String>,
    /// External move (Dancer, etc.)
    pub external_move: bool,
    /// Max move override
    pub max_move: Option<String>,
    /// Original target for redirection tracking
    pub original_target: Option<usize>,
}

/// Use move options for useMove
/// Equivalent to the options parameter in battle-actions.ts useMove()
#[derive(Debug, Clone, Default)]
pub struct UseMoveOptions {
    /// Target pokemon index
    pub target: Option<usize>,
    /// Source effect
    pub source_effect: Option<ID>,
    /// Z-move override
    pub z_move: Option<String>,
    /// Max move override
    pub max_move: Option<String>,
}

/// Spread move damage result type
/// Can be damage amount, false (failed), or true/undefined (success with no damage)
#[derive(Debug, Clone, Copy, Default)]
pub enum SpreadMoveDamageValue {
    Damage(i32),
    Failed,
    Success,
    #[default]
    Undefined,
}

/// Result of spread move hit containing damage and target info
/// JavaScript equivalent: SpreadMoveDamage (sim/global-types.ts)
pub type SpreadMoveDamage = Vec<SpreadMoveDamageValue>;

/// Target info for spread moves (can be the Pokemon or null/false for failed)
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target(usize),
    None,
    Failed,
}

/// Spread move targets array
/// JavaScript equivalent: SpreadMoveTargets (sim/global-types.ts)
pub type SpreadMoveTargets = Vec<SpreadMoveTarget>;

impl<'a> BattleActions<'a> {
}

/// Result of run_move
#[derive(Debug, Clone)]
pub struct RunMoveResult {
    pub move_id: ID,
    pub pokemon_index: usize,
    pub target_loc: i32,
    pub z_move: Option<String>,
    pub max_move: Option<String>,
    pub external_move: bool,
    pub success: bool,
}

/// Result of run_z_power
#[derive(Debug, Clone)]
pub enum ZPowerResult {
    DamageMove,
    Boost(BoostsTable),
    Heal,
    HealReplacement,
    ClearNegativeBoost,
    Redirect,
    Crit2,
    None,
}

/// Result of terastallize
#[derive(Debug, Clone)]
pub enum TerastallizeResult {
    Success {
        tera_type: String,
        forme_change: Option<String>,
    },
    InvalidOgerpon,
}

/// Result of switchIn
/// Equivalent to the return value of battle-actions.ts switchIn()
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchInResult {
    /// Switch was successful
    Success,
    /// Switch was blocked (e.g., by an event returning false)
    Blocked,
    /// Pokemon fainted from Pursuit before switching
    PursuitFaint,
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost_modifier() {
        assert_eq!(BattleActions::get_boost_modifier(0), (2, 2));
        assert_eq!(BattleActions::get_boost_modifier(1), (3, 2));
        assert_eq!(BattleActions::get_boost_modifier(2), (4, 2));
        assert_eq!(BattleActions::get_boost_modifier(-1), (2, 3));
        assert_eq!(BattleActions::get_boost_modifier(6), (8, 2));
        assert_eq!(BattleActions::get_boost_modifier(-6), (2, 8));
    }

    #[test]
    fn test_recoil_damage() {
        // Chloroblast uses 50% HP
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "chloroblast", None, 200),
            100
        );

        // Normal recoil (1/4)
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "doubleedge", Some((1, 4)), 200),
            25
        );

        // No recoil
        assert_eq!(
            BattleActions::calc_recoil_damage(100, "tackle", None, 200),
            0
        );
    }

    #[test]
    fn test_confusion_damage() {
        // Level 50, 100 atk, 100 def, 40 base power, 100% random
        let damage = BattleActions::get_confusion_damage(50, 100, 100, 40, 100);
        assert!(damage > 0);
    }

    #[test]
    fn test_target_type_choices() {
        assert!(BattleActions::target_type_choices("normal"));
        assert!(BattleActions::target_type_choices("any"));
        assert!(BattleActions::target_type_choices("adjacentFoe"));
        assert!(!BattleActions::target_type_choices("self"));
        assert!(!BattleActions::target_type_choices("all"));
    }

    // TODO: Re-enable once get_max_move_name is implemented
    // #[test]
    // fn test_max_move_name() {
    //     assert_eq!(get_max_move_name("Fire"), "Max Flare");
    //     assert_eq!(get_max_move_name("Water"), "Max Geyser");
    //     assert_eq!(get_max_move_name("Electric"), "Max Lightning");
    //     assert_eq!(get_max_move_name("Status"), "Max Guard");
    // }

    // TODO: Re-enable once get_z_move_name is implemented
    // #[test]
    // fn test_z_move_name() {
    //     assert_eq!(get_z_move_name("Fire"), "Inferno Overdrive");
    //     assert_eq!(get_z_move_name("Water"), "Hydro Vortex");
    //     assert_eq!(get_z_move_name("Electric"), "Gigavolt Havoc");
    // }

    #[test]
    fn test_can_ultra_burst() {
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma-Dawn-Wings", "ultranecroziumz"),
            Some("Necrozma-Ultra".to_string())
        );
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma-Dusk-Mane", "ultranecroziumz"),
            Some("Necrozma-Ultra".to_string())
        );
        assert_eq!(
            BattleActions::can_ultra_burst("Necrozma", "ultranecroziumz"),
            None
        );
    }

    #[test]
    fn test_can_terastallize() {
        // Gen 9 with tera type
        assert_eq!(
            BattleActions::can_terastallize(false, false, 9, Some("Fire")),
            Some("Fire".to_string())
        );

        // Wrong gen
        assert_eq!(
            BattleActions::can_terastallize(false, false, 8, Some("Fire")),
            None
        );

        // Has Z-Move
        assert_eq!(
            BattleActions::can_terastallize(true, false, 9, Some("Fire")),
            None
        );

        // Can Mega Evo
        assert_eq!(
            BattleActions::can_terastallize(false, true, 9, Some("Fire")),
            None
        );
    }
}
