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
mod get_max_move_name;
mod get_z_move_name;
mod use_move;
mod use_move_inner;
mod switch_in;
mod drag_in;
mod run_switch;
mod get_damage;
pub use get_max_move_name::get_max_move_name;
pub use get_z_move_name::get_z_move_name;
pub use use_move::use_move;
pub use use_move_inner::use_move_inner;
pub use switch_in::switch_in;
pub use drag_in::drag_in;
pub use run_switch::run_switch;
pub use get_damage::get_damage;

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
pub struct MoveHitData {
    pub crit: bool,
    pub type_mod: i32,
    pub damage: i32,
    pub z_broke_protect: bool,
}

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ActiveMove {
    pub id: ID,
    pub name: String,
    pub base_power: i32,
    pub category: String,
    pub move_type: String,
    pub accuracy: i32,
    pub priority: i8,
    pub target: String,
    pub flags: MoveFlags,

    // Active state
    pub hit: i32,
    pub total_damage: i32,
    pub spread_hit: bool,
    pub is_external: bool,
    pub is_z: bool,
    pub is_max: bool,
    pub is_z_or_max_powered: bool,
    pub prankster_boosted: bool,
    pub has_bounced: bool,
    pub source_effect: Option<ID>,
    pub ignore_ability: bool,
    pub ignore_immunity: Option<bool>,
    pub ignore_accuracy: bool,
    pub ignore_evasion: bool,
    pub ignore_defensive: bool,
    pub ignore_offensive: bool,
    pub ignore_negative_offensive: bool,
    pub ignore_positive_defensive: bool,
    pub override_offensive_stat: Option<String>,
    pub infiltrates: bool,
    pub will_crit: Option<bool>,
    pub force_stab: bool,
    pub crit_ratio: i32,
    pub crit_modifier: Option<f64>,
    pub self_switch: Option<String>,
    pub self_boost: Option<BoostsTable>,
    pub has_sheer_force: bool,
    pub mindblown_recoil: bool,
    pub struggle_recoil: bool,
    pub self_dropped: bool,
    pub smart_target: Option<bool>,
    pub stellar_boosted: bool,
    pub multi_hit: Option<i32>,
    pub multi_hit_type: Option<String>,
    pub multi_accuracy: bool,
    pub ohko: Option<String>,
    pub always_hit: bool,
    pub breaks_protect: bool,
    pub steals_boosts: bool,
    pub force_switch: bool,
    pub self_destruct: Option<String>,
    pub tracks_target: bool,
    pub base_move: Option<ID>,
    pub max_move: Option<MaxMoveData>,
    pub z_move: Option<ZMoveData>,
    pub sleep_usable: bool,

    // Special move fields
    /// Non-ghost target for Curse (used when ghost type uses it differently)
    pub non_ghost_target: Option<String>,
    /// Whether this move will cause a forme change (relicsong)
    pub will_change_forme: bool,

    // Secondary effects
    pub secondaries: Vec<SecondaryEffect>,
    pub self_effect: Option<SelfEffect>,

    // Move data effects
    pub boosts: Option<BoostsTable>,
    pub heal: Option<(i32, i32)>,
    pub status: Option<String>,
    pub force_status: Option<String>,
    pub volatile_status: Option<String>,
    pub side_condition: Option<String>,
    pub slot_condition: Option<String>,
    pub weather: Option<String>,
    pub terrain: Option<String>,
    pub pseudo_weather: Option<String>,

    // Recoil
    pub recoil: Option<(i32, i32)>,

    // Hit targets (populated during execution)
    pub hit_targets: Vec<(usize, usize)>,
}

/// Move flags
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MoveFlags {
    pub contact: bool,
    pub protect: bool,
    pub mirror: bool,
    pub punch: bool,
    pub bite: bool,
    pub sound: bool,
    pub powder: bool,
    pub dance: bool,
    pub pulse: bool,
    pub bullet: bool,
    pub slicing: bool,
    pub wind: bool,
    pub cant_use_twice: bool,
    pub future_move: bool,
    pub reflectable: bool,
    pub snatch: bool,
    pub gravity: bool,
    pub bypasssub: bool,
    pub pledgecombo: bool,
}

/// Max move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MaxMoveData {
    pub base_power: i32,
}

/// Z-move data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ZMoveData {
    pub base_power: Option<i32>,
    pub boost: Option<BoostsTable>,
    pub effect: Option<String>,
}

/// Secondary effect data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SecondaryEffect {
    pub chance: Option<i32>,
    pub boosts: Option<BoostsTable>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub self_effect: bool,
}

/// Self effect data
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SelfEffect {
    pub boosts: Option<BoostsTable>,
    pub chance: Option<i32>,
    pub side_condition: Option<String>,
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
pub struct BattleActions<'a> {
    pub dex: &'a Dex,
    pub gen: u8,
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
pub type SpreadMoveDamage = Vec<SpreadMoveDamageValue>;

/// Target info for spread moves (can be the Pokemon or null/false for failed)
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target(usize),
    None,
    Failed,
}

/// Spread move targets array
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

    #[test]
    fn test_max_move_name() {
        assert_eq!(get_max_move_name("Fire"), "Max Flare");
        assert_eq!(get_max_move_name("Water"), "Max Geyser");
        assert_eq!(get_max_move_name("Electric"), "Max Lightning");
        assert_eq!(get_max_move_name("Status"), "Max Guard");
    }

    #[test]
    fn test_z_move_name() {
        assert_eq!(get_z_move_name("Fire"), "Inferno Overdrive");
        assert_eq!(get_z_move_name("Water"), "Hydro Vortex");
        assert_eq!(get_z_move_name("Electric"), "Gigavolt Havoc");
    }

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
