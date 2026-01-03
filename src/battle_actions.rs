//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This is a 1:1 port of sim/battle-actions.ts
//! Handles all battle actions: moves, switches, damage calculation, etc.

use std::collections::{HashSet, HashMap};
use serde::{Deserialize, Serialize};

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
mod can_mega_evo;
mod can_ultra_burst;
mod run_mega_evo;
mod can_terastallize;
mod terastallize;
mod run_z_power;
pub mod use_move;
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
mod get_z_move;
mod can_z_move;
mod get_active_z_move;
mod get_max_move;
mod get_active_max_move;
mod after_move_secondary_event;
mod force_switch;
mod hit_step_break_protect;
mod hit_step_invulnerability_event;
mod hit_step_steal_boosts;
mod hit_step_try_hit_event;
mod hit_step_try_immunity;
mod hit_step_type_immunity;
pub use can_mega_evo::can_mega_evo;
pub use can_ultra_burst::can_ultra_burst;
pub use can_terastallize::can_terastallize;
pub use get_confusion_damage::get_confusion_damage;
pub use run_mega_evo::run_mega_evo;
pub use terastallize::terastallize;
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
pub use get_z_move::get_z_move;
pub use can_z_move::can_z_move;
pub use get_active_z_move::get_active_z_move;
pub use get_max_move::get_max_move;
pub use get_active_max_move::get_active_max_move;
pub use after_move_secondary_event::after_move_secondary_event;
pub use force_switch::force_switch;
pub use hit_step_break_protect::hit_step_break_protect;
pub use hit_step_invulnerability_event::hit_step_invulnerability_event;
pub use hit_step_steal_boosts::hit_step_steal_boosts;
pub use hit_step_try_hit_event::hit_step_try_hit_event;
pub use hit_step_try_immunity::hit_step_try_immunity;
pub use hit_step_type_immunity::hit_step_type_immunity;

/// Choosable target types for moves
/// JavaScript equivalent: CHOOSABLE_TARGETS constant (sim/battle-actions.ts)
/// JavaScript: Set(['normal', 'any', 'adjacentAlly', 'adjacentAllyOrSelf', 'adjacentFoe'])
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
/// TODO: Not in JavaScript - Rust-specific struct for passing Z-move parameters
/// JavaScript uses inline parameters in Z-move related functions
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
/// TODO: Not in JavaScript - Rust-specific struct for passing can_z_move parameters
/// JavaScript uses inline parameters in canZMove function
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
/// TODO: Not in JavaScript - Rust-specific struct for passing damage calculation parameters
/// JavaScript uses inline parameters in damage calculation functions
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
/// TODO: Not in JavaScript - Rust-specific enum for damage calculation results
/// JavaScript functions return number (damage) | undefined (immune/miss/fail) | false (failed)
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

/// Ignore immunity setting for moves
/// JavaScript: ignoreImmunity?: boolean | { [k: string]: boolean }
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IgnoreImmunity {
    /// Ignore all type immunities (true)
    All,
    /// Ignore specific type immunities ({ Type: true, ... })
    Specific(HashMap<String, bool>),
}

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: ActiveMove (sim/dex-moves.ts)
/// 35+ fields in JavaScript (plus inherited from MutableMove which is BasicEffect & MoveData & HitEffect)
pub struct ActiveMove {
    /// Effect type constant (always "Move")
    /// JavaScript: readonly effectType: 'Move'
    // This is implicit in Rust - all ActiveMove instances are moves

    // =========================================================================
    // From BasicEffect (inherited via MutableMove)
    // =========================================================================
    /// Move ID
    /// JavaScript: id: ID
    pub id: ID,
    /// Move name
    /// JavaScript: name: string
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
    /// Base power of the move
    /// JavaScript: basePower: number
    pub base_power: i32,
    /// Accuracy value
    /// JavaScript: accuracy: true | number
    /// Rust: Uses Accuracy enum with Percent(i32) and AlwaysHits variants
    pub accuracy: crate::dex::Accuracy,
    /// Power Points
    /// JavaScript: pp: number
    pub pp: u8,
    /// Move category (Physical/Special/Status)
    /// JavaScript: category: string
    pub category: String,
    /// Move type
    /// JavaScript: type: string
    pub move_type: String,
    /// Move priority (-7 to +7)
    /// JavaScript: priority: number
    pub priority: i8,
    /// Target type
    /// JavaScript: target: string
    pub target: String,
    /// Move flags
    /// JavaScript: flags: MoveFlags
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
    /// Forces the target to switch out
    /// JavaScript: forceSwitch: boolean
    pub force_switch: bool,
    /// Self-switch effect
    /// JavaScript: selfSwitch?: 'copyvolatile' | 'shedtail' | boolean
    /// TODO: Rust uses Option<String>, cannot fully represent boolean variant
    pub self_switch: Option<String>,
    /// Self stat boosts (e.g., Bulk Up)
    /// JavaScript: selfBoost?: BoostsTable
    pub self_boost: Option<BoostsTable>,
    /// Self-destruct type
    /// JavaScript: selfdestruct?: 'always' | 'ifHit' | boolean
    /// TODO: Rust uses Option<String>, cannot fully represent boolean variant
    pub self_destruct: Option<String>,
    /// Breaks through protection
    /// JavaScript: breaksProtect: boolean
    pub breaks_protect: bool,
    /// Recoil damage fraction
    /// JavaScript: recoil?: [number, number]
    pub recoil: Option<(i32, i32)>,
    /// Mind Blown recoil flag
    /// JavaScript: mindBlownRecoil: boolean
    pub mindblown_recoil: bool,
    /// Steals target's stat boosts
    /// JavaScript: stealsBoosts: boolean
    pub steals_boosts: bool,
    /// Struggle recoil flag
    /// JavaScript: struggleRecoil: boolean
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
    /// Has Sheer Force flag
    /// JavaScript: hasSheerForce: boolean
    pub has_sheer_force: bool,
    /// Always hits (ignores accuracy)
    /// JavaScript: alwaysHit: boolean
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
    /// Critical hit ratio (1 = normal, 2 = high crit rate)
    /// JavaScript: critRatio: number
    pub crit_ratio: i32,
    /// Override which Pokemon's offensive stat to use
    /// JavaScript: overrideOffensivePokemon?: 'target' | 'source'
    pub override_offensive_pokemon: Option<String>,
    /// Override which offensive stat to use
    /// JavaScript: overrideOffensiveStat?: StatIDExceptHP
    pub override_offensive_stat: Option<String>,
    /// Override which Pokemon's defensive stat to use
    /// JavaScript: overrideDefensivePokemon?: 'target' | 'source'
    pub override_defensive_pokemon: Option<String>,
    /// Override which defensive stat to use
    /// JavaScript: overrideDefensiveStat?: StatIDExceptHP
    pub override_defensive_stat: Option<String>,
    /// Force STAB (Same Type Attack Bonus)
    /// JavaScript: forceSTAB: boolean
    pub force_stab: bool,
    /// Ignore target's ability
    /// JavaScript: ignoreAbility: boolean
    pub ignore_ability: bool,
    /// Ignore accuracy checks
    /// JavaScript: ignoreAccuracy: boolean
    pub ignore_accuracy: bool,
    /// Ignore target's evasion
    /// JavaScript: ignoreEvasion: boolean
    pub ignore_evasion: bool,
    /// Ignore positive evasion boosts
    /// JavaScript: ignorePositiveEvasion?: boolean
    pub ignore_positive_evasion: Option<bool>,
    /// Ignore type immunity
    /// JavaScript: ignoreImmunity?: boolean | { [k: string]: boolean }
    pub ignore_immunity: Option<IgnoreImmunity>,
    /// Ignore defensive stat changes
    /// JavaScript: ignoreDefensive: boolean
    pub ignore_defensive: bool,
    /// Ignore offensive stat changes
    /// JavaScript: ignoreOffensive: boolean
    pub ignore_offensive: bool,
    /// Ignore negative offensive stat changes
    /// JavaScript: ignoreNegativeOffensive: boolean
    pub ignore_negative_offensive: bool,
    /// Ignore positive defensive stat changes
    /// JavaScript: ignorePositiveDefensive: boolean
    pub ignore_positive_defensive: bool,
    /// Infiltrates through Substitute, etc.
    /// JavaScript: infiltrates: boolean
    pub infiltrates: bool,
    /// Will always crit
    /// JavaScript: willCrit?: boolean
    pub will_crit: Option<bool>,
    /// Multi-accuracy flag (for moves like Triple Axel)
    /// JavaScript: multiaccuracy: boolean
    pub multi_accuracy: bool,
    /// Number of hits for multi-hit moves
    /// JavaScript: multihit?: number | number[]
    /// TODO: Rust uses Option<i32>, cannot represent number[] variant
    pub multi_hit: Option<i32>,
    /// Multi-hit type (e.g., "parentalbond")
    /// JavaScript: multihitType?: string
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
    /// Usable while sleeping
    /// JavaScript: sleepUsable: boolean
    pub sleep_usable: bool,
    /// Smart target selection
    /// JavaScript: smartTarget?: boolean
    pub smart_target: Option<bool>,
    /// Tracks target (e.g., Lock-On)
    /// JavaScript: tracksTarget: boolean
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
    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    /// JavaScript: status?: string
    pub status: Option<String>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
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
    /// Terrain to set
    /// JavaScript: terrain?: string
    pub terrain: Option<String>,
    /// Weather to set
    /// JavaScript: weather?: string
    pub weather: Option<String>,

    // =========================================================================
    // ActiveMove-specific fields
    // =========================================================================
    /// Hit number in multi-hit moves
    /// JavaScript: hit: number
    pub hit: i32,
    /// Total damage dealt
    /// JavaScript: totalDamage: number | false
    /// TODO: Rust uses i32, cannot represent false variant
    pub total_damage: i32,
    /// Move hit data for tracking crit, type effectiveness, etc.
    /// JavaScript: moveHitData?: MoveHitData
    pub move_hit_data: HashMap<String, crate::pokemon::MoveHitData>,
    /// Hit a spread move (doubles/triples)
    /// JavaScript: spreadHit: boolean
    pub spread_hit: bool,
    /// Last hit of a multi-hit move
    /// JavaScript: lastHit?: boolean
    pub last_hit: Option<bool>,
    /// Move is from external source (not direct Pokemon action)
    /// JavaScript: isExternal: boolean
    pub is_external: bool,
    /// Is Z or Max powered
    /// JavaScript: isZOrMaxPowered: boolean
    pub is_z_or_max_powered: bool,
    /// Prankster boosted this move
    /// JavaScript: pranksterBoosted: boolean
    pub prankster_boosted: bool,
    /// Move has been bounced by Magic Coat/Bounce
    /// JavaScript: hasBounced: boolean
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
    /// User dropped its item (Fling, etc.)
    /// JavaScript: selfDropped: boolean
    pub self_dropped: bool,
    /// Move is Stellar type boosted
    /// JavaScript: stellarBoosted: boolean
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
    /// JavaScript: willChangeForme: boolean
    pub will_change_forme: bool,
    /// Status roll result for secondary status effects
    /// JavaScript: statusRoll?: string
    pub status_roll: Option<String>,
    /// Force a specific status condition
    /// JavaScript: forceStatus?: string
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
    /// Ally-targeting animation flag
    /// JavaScript: allyanim?: 1
    pub allyanim: bool,
    /// Bite flag (affected by Strong Jaw)
    /// JavaScript: bite?: 1
    pub bite: bool,
    /// Bullet flag (blocked by Bulletproof)
    /// JavaScript: bullet?: 1
    pub bullet: bool,
    /// Bypass Substitute flag
    /// JavaScript: bypasssub?: 1
    pub bypasssub: bool,
    /// Can't use twice flag (e.g., Dynamax Cannon)
    /// JavaScript: cantusetwice?: 1
    pub cant_use_twice: bool,
    /// Charge flag (two-turn moves)
    /// JavaScript: charge?: 1
    pub charge: bool,
    /// Contact flag (makes physical contact)
    /// JavaScript: contact?: 1
    pub contact: bool,
    /// Dance flag (copied by Dancer)
    /// JavaScript: dance?: 1
    pub dance: bool,
    /// Defrost flag (thaws user)
    /// JavaScript: defrost?: 1
    pub defrost: bool,
    /// Distance flag (blocked by Wide Guard)
    /// JavaScript: distance?: 1
    pub distance: bool,
    /// Fail if used by Copycat
    /// JavaScript: failcopycat?: 1
    pub failcopycat: bool,
    /// Fail if used by Encore
    /// JavaScript: failencore?: 1
    pub failencore: bool,
    /// Fail if used by Instruct
    /// JavaScript: failinstruct?: 1
    pub failinstruct: bool,
    /// Fail if used by Me First
    /// JavaScript: failmefirst?: 1
    pub failmefirst: bool,
    /// Fail if used by Mimic
    /// JavaScript: failmimic?: 1
    pub failmimic: bool,
    /// Future move flag (Future Sight, Doom Desire)
    /// JavaScript: futuremove?: 1
    pub future_move: bool,
    /// Affected by Gravity
    /// JavaScript: gravity?: 1
    pub gravity: bool,
    /// Heal flag (blocked by Heal Block)
    /// JavaScript: heal?: 1
    pub heal: bool,
    /// Can be used by Metronome
    /// JavaScript: metronome?: 1
    pub metronome: bool,
    /// Mirror Move flag (can be copied)
    /// JavaScript: mirror?: 1
    pub mirror: bool,
    /// Must pressure flag (must deduct PP under Pressure)
    /// JavaScript: mustpressure?: 1
    pub mustpressure: bool,
    /// Blocked by Assist
    /// JavaScript: noassist?: 1
    pub noassist: bool,
    /// Cannot be used in Sky Battles
    /// JavaScript: nonsky?: 1
    pub nonsky: bool,
    /// Not affected by Parental Bond
    /// JavaScript: noparentalbond?: 1
    pub noparentalbond: bool,
    /// Cannot be Sketched
    /// JavaScript: nosketch?: 1
    pub nosketch: bool,
    /// Cannot be used by Sleep Talk
    /// JavaScript: nosleeptalk?: 1
    pub nosleeptalk: bool,
    /// Pledge combo flag
    /// JavaScript: pledgecombo?: 1
    pub pledgecombo: bool,
    /// Powder flag (blocked by Grass types and Overcoat)
    /// JavaScript: powder?: 1
    pub powder: bool,
    /// Protect flag (blocked by protection moves)
    /// JavaScript: protect?: 1
    pub protect: bool,
    /// Pulse flag (boosted by Mega Launcher)
    /// JavaScript: pulse?: 1
    pub pulse: bool,
    /// Punch flag (boosted by Iron Fist)
    /// JavaScript: punch?: 1
    pub punch: bool,
    /// Recharge flag (requires recharge next turn)
    /// JavaScript: recharge?: 1
    pub recharge: bool,
    /// Reflectable flag (can be bounced by Magic Coat)
    /// JavaScript: reflectable?: 1
    pub reflectable: bool,
    /// Slicing flag (boosted by Sharpness)
    /// JavaScript: slicing?: 1
    pub slicing: bool,
    /// Snatch flag (can be stolen by Snatch)
    /// JavaScript: snatch?: 1
    pub snatch: bool,
    /// Sound flag (blocked by Soundproof)
    /// JavaScript: sound?: 1
    pub sound: bool,
    /// Wind flag (boosted by Wind Power)
    /// JavaScript: wind?: 1
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
    /// JavaScript: chance?: number
    pub chance: Option<i32>,

    // From HitEffect:
    /// onHit callback
    /// JavaScript: onHit?: MoveEventMethods['onHit']
    /// Note: Cannot store function reference in Rust, would need callback system
    // pub on_hit: Option<...>,

    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    /// JavaScript: status?: string
    pub status: Option<String>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
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
/// 9 fields in JavaScript HitEffect
pub struct SelfEffect {
    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    pub boosts: Option<BoostsTable>,
    /// Status condition to inflict
    /// JavaScript: status?: string
    pub status: Option<String>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus")]
    pub volatile_status: Option<String>,
    /// Side condition to apply
    /// JavaScript: sideCondition?: string
    #[serde(rename = "sideCondition")]
    pub side_condition: Option<String>,
    /// Slot condition to apply
    /// JavaScript: slotCondition?: string
    #[serde(rename = "slotCondition")]
    pub slot_condition: Option<String>,
    /// Pseudo-weather to apply
    /// JavaScript: pseudoWeather?: string
    #[serde(rename = "pseudoWeather")]
    pub pseudo_weather: Option<String>,
    /// Terrain to apply
    /// JavaScript: terrain?: string
    pub terrain: Option<String>,
    /// Weather to apply
    /// JavaScript: weather?: string
    pub weather: Option<String>,
    /// Chance of the effect occurring (percentage)
    /// JavaScript: chance?: number
    pub chance: Option<i32>,
    // Note: onHit callback cannot be stored in Rust data structures
    // JavaScript: onHit?: MoveEventMethods['onHit']
}

/// Z-Move request option
/// JavaScript equivalent: ZMoveOption (sim/side.ts)
/// 2 fields in JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZMoveOption {
    pub move_name: String,
    pub target: String,
}

/// Damage value (can be number, false, or undefined-like None)
/// TODO: Rust uses enum to represent JavaScript's number | false | undefined union type
/// JavaScript: number | false | undefined
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(i32),
    Failed,
    Blocked, // HIT_SUBSTITUTE
}

/// Switch copy flag type
/// TODO: Not in JavaScript - Rust-specific enum for switch copy behavior
/// JavaScript uses string literals 'copyvolatile' | 'shedtail' | true
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
pub struct BattleActions<'a> {
    // TODO: Add battle reference when lifetime management is resolved
    // pub battle: &'a Battle,

    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
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
/// JavaScript equivalent: Inline return type in battle-actions.ts afterMoveSecondarySelfEffects()
/// 2 fields in JavaScript
pub struct AfterMoveResult {
    /// Self-switch effect type
    /// JavaScript: selfSwitch?: 'copyvolatile' | 'shedtail' | true
    pub self_switch: Option<String>,
    /// Force switch flag
    /// JavaScript: forceSwitch?: boolean
    pub force_switch: bool,
}

/// Move effects to apply
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: Inline type in battle-actions.ts
/// 4 fields in JavaScript
pub struct MoveEffects {
    /// Stat boosts to apply
    /// JavaScript: boosts?: SparseBoostsTable
    pub boosts: Option<BoostsTable>,
    /// Healing fraction
    /// JavaScript: heal?: [number, number]
    pub heal: Option<(i32, i32)>,
    /// Status condition to inflict
    /// JavaScript: status?: string
    pub status: Option<String>,
    /// Volatile status to inflict
    /// JavaScript: volatileStatus?: string
    pub volatile_status: Option<String>,
}

/// Run move options for runMove
/// Equivalent to the options parameter in battle-actions.ts runMove()
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: RunMoveOptions (sim/battle-actions.ts)
/// 5 fields in JavaScript
pub struct RunMoveOptions {
    /// Source effect that caused this move
    /// JavaScript: sourceEffect?: Effect
    pub source_effect: Option<ID>,
    /// Z-move override
    /// JavaScript: zMove?: string
    pub z_move: Option<String>,
    /// External move (Dancer, etc.)
    /// JavaScript: externalMove?: boolean
    pub external_move: bool,
    /// Max move override
    /// JavaScript: maxMove?: string
    pub max_move: Option<String>,
    /// Original target for redirection tracking
    /// JavaScript: originalTarget?: Pokemon
    pub original_target: Option<usize>,
}

/// Use move options for useMove
/// Equivalent to the options parameter in battle-actions.ts useMove()
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: UseMoveOptions (sim/battle-actions.ts)
/// 4 fields in JavaScript
pub struct UseMoveOptions {
    /// Target pokemon index
    /// JavaScript: target?: Pokemon
    pub target: Option<usize>,
    /// Source effect
    /// JavaScript: sourceEffect?: Effect
    pub source_effect: Option<ID>,
    /// Z-move override
    /// JavaScript: zMove?: string
    pub z_move: Option<String>,
    /// Max move override
    /// JavaScript: maxMove?: string
    pub max_move: Option<String>,
}

/// Spread move damage result type
/// Can be damage amount, false (failed), or true/undefined (success with no damage)
/// TODO: Rust uses enum to represent JavaScript's number | false | true | undefined union type
/// JavaScript: number | false | true | undefined
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
/// TODO: Rust uses enum to represent JavaScript's Pokemon | null | false union type
/// JavaScript: Pokemon | null | false
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target((usize, usize)),  // (side_index, pokemon_index)
    None,
    Failed,
}

/// Spread move targets array
/// JavaScript equivalent: SpreadMoveTargets (sim/global-types.ts)
pub type SpreadMoveTargets = Vec<SpreadMoveTarget>;

impl<'a> BattleActions<'a> {
}

/// Result of run_move
/// TODO: Not in JavaScript - Rust-specific struct for runMove return value
/// JavaScript runMove() returns boolean (success/failure)
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
/// TODO: Not in JavaScript - Rust-specific enum for runZPower return value
/// JavaScript runZPower() uses various return patterns (boolean, object, etc.)
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
/// TODO: Not in JavaScript - Rust-specific enum for terastallize return value
/// JavaScript terastallize() returns various values (string, boolean, object)
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

    // TODO: Update these tests to use the new signature
    // #[test]
    // fn test_can_ultra_burst() {
    //     assert_eq!(
    //         BattleActions::can_ultra_burst("Necrozma-Dawn-Wings", "ultranecroziumz"),
    //         Some("Necrozma-Ultra".to_string())
    //     );
    //     assert_eq!(
    //         BattleActions::can_ultra_burst("Necrozma-Dusk-Mane", "ultranecroziumz"),
    //         Some("Necrozma-Ultra".to_string())
    //     );
    //     assert_eq!(
    //         BattleActions::can_ultra_burst("Necrozma", "ultranecroziumz"),
    //         None
    //     );
    // }

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
