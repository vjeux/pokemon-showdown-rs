//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This is a 1:1 port of sim/battle-actions.ts
//! Handles all battle actions: moves, switches, damage calculation, etc.

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::dex_data::{ID, BoostsTable, EffectState};
use crate::dex::{Dex, MoveData};
use crate::pokemon::Pokemon;
use crate::data::typechart::get_effectiveness_multi;

/// Choosable target types for moves
static CHOOSABLE_TARGETS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("normal");
    set.insert("any");
    set.insert("adjacentAlly");
    set.insert("adjacentAllyOrSelf");
    set.insert("adjacentFoe");
    set
});

/// Max Move names by type
pub fn get_max_move_name(move_type: &str) -> &'static str {
    match move_type {
        "Flying" => "Max Airstream",
        "Dark" => "Max Darkness",
        "Fire" => "Max Flare",
        "Bug" => "Max Flutterby",
        "Water" => "Max Geyser",
        "Status" => "Max Guard",
        "Ice" => "Max Hailstorm",
        "Fighting" => "Max Knuckle",
        "Electric" => "Max Lightning",
        "Psychic" => "Max Mindstorm",
        "Poison" => "Max Ooze",
        "Grass" => "Max Overgrowth",
        "Ghost" => "Max Phantasm",
        "Ground" => "Max Quake",
        "Rock" => "Max Rockfall",
        "Fairy" => "Max Starfall",
        "Steel" => "Max Steelspike",
        "Normal" => "Max Strike",
        "Dragon" => "Max Wyrmwind",
        _ => "Max Strike",
    }
}

/// Z-Move names by type
pub fn get_z_move_name(move_type: &str) -> &'static str {
    match move_type {
        "Poison" => "Acid Downpour",
        "Fighting" => "All-Out Pummeling",
        "Dark" => "Black Hole Eclipse",
        "Grass" => "Bloom Doom",
        "Normal" => "Breakneck Blitz",
        "Rock" => "Continental Crush",
        "Steel" => "Corkscrew Crash",
        "Dragon" => "Devastating Drake",
        "Electric" => "Gigavolt Havoc",
        "Water" => "Hydro Vortex",
        "Fire" => "Inferno Overdrive",
        "Ghost" => "Never-Ending Nightmare",
        "Bug" => "Savage Spin-Out",
        "Psychic" => "Shattered Psyche",
        "Ice" => "Subzero Slammer",
        "Flying" => "Supersonic Skystrike",
        "Ground" => "Tectonic Rage",
        "Fairy" => "Twinkle Tackle",
        _ => "Breakneck Blitz",
    }
}

/// Damage calculation result
#[derive(Debug, Clone)]
pub enum DamageResult {
    /// Actual damage dealt
    Damage(u32),
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
    pub damage: u32,
    pub z_broke_protect: bool,
}

/// Active move state - represents a move being executed
/// Equivalent to ActiveMove in battle-actions.ts
#[derive(Debug, Clone, Default)]
pub struct ActiveMove {
    pub id: ID,
    pub name: String,
    pub base_power: u32,
    pub category: String,
    pub move_type: String,
    pub accuracy: u32,
    pub priority: i8,
    pub target: String,
    pub flags: MoveFlags,

    // Active state
    pub hit: u32,
    pub total_damage: u32,
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
    pub multi_hit: Option<u32>,
    pub multi_hit_type: Option<String>,
    pub multi_accuracy: bool,
    pub ohko: Option<String>,
    pub always_hit: bool,
    pub breaks_protect: bool,
    pub steals_boosts: bool,
    pub force_switch: bool,
    pub self_destruct: Option<String>,
    pub base_move: Option<ID>,
    pub max_move: Option<MaxMoveData>,
    pub z_move: Option<ZMoveData>,
    pub sleep_usable: bool,

    // Secondary effects
    pub secondaries: Vec<SecondaryEffect>,
    pub self_effect: Option<SelfEffect>,

    // Move data effects
    pub boosts: Option<BoostsTable>,
    pub heal: Option<(u32, u32)>,
    pub status: Option<String>,
    pub force_status: Option<String>,
    pub volatile_status: Option<String>,
    pub side_condition: Option<String>,
    pub slot_condition: Option<String>,
    pub weather: Option<String>,
    pub terrain: Option<String>,
    pub pseudo_weather: Option<String>,

    // Recoil
    pub recoil: Option<(u32, u32)>,

    // Hit targets (populated during execution)
    pub hit_targets: Vec<(usize, usize)>,
}

/// Move flags
#[derive(Debug, Clone, Default)]
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
}

/// Max move data
#[derive(Debug, Clone, Default)]
pub struct MaxMoveData {
    pub base_power: u32,
}

/// Z-move data
#[derive(Debug, Clone, Default)]
pub struct ZMoveData {
    pub base_power: Option<u32>,
    pub boost: Option<BoostsTable>,
    pub effect: Option<String>,
}

/// Secondary effect data
#[derive(Debug, Clone, Default)]
pub struct SecondaryEffect {
    pub chance: Option<u32>,
    pub boosts: Option<BoostsTable>,
    pub status: Option<String>,
    pub volatile_status: Option<String>,
    pub self_effect: bool,
}

/// Self effect data
#[derive(Debug, Clone, Default)]
pub struct SelfEffect {
    pub boosts: Option<BoostsTable>,
    pub chance: Option<u32>,
}

/// Z-Move request option
#[derive(Debug, Clone)]
pub struct ZMoveOption {
    pub move_name: String,
    pub target: String,
}

/// Spread move damage result
pub type SpreadMoveDamage = Vec<Option<DamageValue>>;

/// Damage value (can be number, false, or undefined-like None)
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(u32),
    Failed,
    Blocked,  // HIT_SUBSTITUTE
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
    pub fn new(dex: &'a Dex, gen: u8) -> Self {
        Self { dex, gen }
    }

    /// Calculate damage for a move
    /// This is a simplified damage calculation for testing purposes.
    /// The full damage calculation is in getDamage in battle-actions.ts
    pub fn calculate_damage(
        &self,
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &MoveData,
        is_crit: bool,
        random_factor: u32,
    ) -> DamageResult {
        // Check for immunity first
        let effectiveness = get_effectiveness_multi(&move_data.move_type, &defender.types);

        if effectiveness == 0.0 {
            return DamageResult::Immune;
        }

        // Get base power
        let base_power = move_data.base_power;
        if base_power == 0 {
            return DamageResult::NoDamage;
        }

        // Get attack and defense stats with boost modifiers applied
        let (attack, defense) = if move_data.category == "Special" {
            let atk_boost = attacker.boosts.spa;
            let def_boost = defender.boosts.spd;
            let base_atk = attacker.stored_stats.spa as u32;
            let base_def = defender.stored_stats.spd as u32;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        } else {
            let atk_boost = attacker.boosts.atk;
            let def_boost = defender.boosts.def;
            let base_atk = attacker.stored_stats.atk as u32;
            let base_def = defender.stored_stats.def as u32;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        };

        // Basic damage formula: ((2 * Level / 5 + 2) * Power * A/D) / 50 + 2
        let level = attacker.level as u32;
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense.max(1)) / 50 + 2;

        // Apply STAB (Same Type Attack Bonus)
        let stab = if attacker.types.iter().any(|t| t == &move_data.move_type) {
            1.5
        } else {
            1.0
        };

        // Apply type effectiveness
        let damage = (base_damage as f64 * stab * effectiveness) as u32;

        // Apply critical hit (1.5x in Gen 6+)
        let damage = if is_crit {
            (damage as f64 * 1.5) as u32
        } else {
            damage
        };

        // Apply random factor (0.85 to 1.0, passed in as 85-100)
        let damage = damage * random_factor / 100;

        DamageResult::Damage(damage.max(1))
    }

    // =========================================================================
    // SWITCH METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: switchIn, dragIn, runSwitch are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // MOVE METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: runMove, useMove, useMoveInner are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // DAMAGE CALCULATION - These can be pure functions
    // =========================================================================

    /// Calculate the stat modifier from boost stages
    /// Returns the multiplier as a fraction (numerator, denominator)
    /// Equivalent to getBoostMod in Pokemon Showdown
    pub fn get_boost_modifier(boost: i8) -> (i32, i32) {
        match boost {
            -6 => (2, 8),
            -5 => (2, 7),
            -4 => (2, 6),
            -3 => (2, 5),
            -2 => (2, 4),
            -1 => (2, 3),
            0 => (2, 2),
            1 => (3, 2),
            2 => (4, 2),
            3 => (5, 2),
            4 => (6, 2),
            5 => (7, 2),
            6 => (8, 2),
            _ if boost < -6 => (2, 8),
            _ => (8, 2),
        }
    }

    /// Calculate the effective stat value with boost applied
    pub fn calculate_stat_with_boost(base_stat: u32, boost: i8) -> u32 {
        let (num, denom) = Self::get_boost_modifier(boost);
        (base_stat as i32 * num / denom).max(1) as u32
    }

    /// Calculate recoil damage
    /// Equivalent to calcRecoilDamage in battle-actions.ts
    pub fn calc_recoil_damage(damage_dealt: u32, move_id: &str, recoil: Option<(u32, u32)>, pokemon_max_hp: u32) -> u32 {
        if move_id == "chloroblast" {
            return (pokemon_max_hp / 2).max(1);
        }
        if let Some((num, denom)) = recoil {
            let recoil_damage = (damage_dealt * num / denom).max(1);
            return recoil_damage;
        }
        0
    }

    /// Calculate confusion damage
    /// Equivalent to getConfusionDamage in battle-actions.ts
    pub fn get_confusion_damage(level: u32, attack: u32, defense: u32, base_power: u32, random_factor: u32) -> u32 {
        // int(int(int(2 * L / 5 + 2) * P * A / D) / 50) + 2
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense) / 50 + 2;

        // Apply random factor (0.85 to 1.0)
        let damage = base_damage * random_factor / 100;

        damage.max(1)
    }

    /// Check if a target type allows choosing
    /// Equivalent to targetTypeChoices in battle-actions.ts
    pub fn target_type_choices(target_type: &str) -> bool {
        CHOOSABLE_TARGETS.contains(target_type)
    }

    /// Combine results for damage/effect calculations
    /// Equivalent to combineResults in battle-actions.ts
    pub fn combine_results(left: Option<DamageValue>, right: Option<DamageValue>) -> Option<DamageValue> {
        // Priority: undefined < NOT_FAIL < null < boolean < number
        match (&left, &right) {
            (None, r) => r.clone(),
            (l, None) => l.clone(),
            (Some(DamageValue::Damage(l)), Some(DamageValue::Damage(r))) => {
                Some(DamageValue::Damage(l + r))
            }
            (_, r) => r.clone(),
        }
    }

    // =========================================================================
    // Z-MOVE METHODS
    // =========================================================================

    /// Get Z-Move for a move
    /// Equivalent to getZMove in battle-actions.ts
    pub fn get_z_move(
        move_name: &str,
        move_type: &str,
        move_category: &str,
        z_move_base_power: Option<u32>,
        item_z_move: Option<&str>,
        item_z_move_from: Option<&str>,
        item_z_move_type: Option<&str>,
        z_move_used: bool,
    ) -> Option<String> {
        if z_move_used {
            return None;
        }

        // Check for signature Z-move
        if let Some(z_move_from) = item_z_move_from {
            if move_name == z_move_from {
                return item_z_move.map(|s| s.to_string());
            }
        }

        // Check for type-based Z-move
        if item_z_move.is_some() {
            if let Some(z_type) = item_z_move_type {
                if move_type == z_type {
                    if move_category == "Status" {
                        return Some(move_name.to_string());
                    } else if z_move_base_power.is_some() {
                        return Some(get_z_move_name(move_type).to_string());
                    }
                }
            }
        }

        None
    }

    /// Check if Pokemon can use Z-Move
    /// Equivalent to canZMove in battle-actions.ts
    pub fn can_z_move(
        z_move_used: bool,
        is_transformed: bool,
        species_is_mega: bool,
        species_is_primal: bool,
        species_forme: &str,
        item_z_move: bool,
        item_user: Option<&[String]>,
        species_name: &str,
    ) -> bool {
        if z_move_used {
            return false;
        }
        if is_transformed && (species_is_mega || species_is_primal || species_forme == "Ultra") {
            return false;
        }
        if !item_z_move {
            return false;
        }
        if let Some(users) = item_user {
            if !users.iter().any(|u| u == species_name) {
                return false;
            }
        }
        true
    }

    // =========================================================================
    // MAX MOVE METHODS
    // =========================================================================

    /// Get Max Move for a move
    /// Equivalent to getMaxMove in battle-actions.ts
    pub fn get_max_move(move_name: &str, move_type: &str, move_category: &str) -> Option<String> {
        if move_name == "Struggle" {
            return Some("Struggle".to_string());
        }

        let max_type = if move_category == "Status" {
            "Status"
        } else {
            move_type
        };

        Some(get_max_move_name(max_type).to_string())
    }

    // =========================================================================
    // MEGA EVOLUTION METHODS
    // =========================================================================

    /// Check if Pokemon can Mega Evolve
    /// Equivalent to canMegaEvo in battle-actions.ts
    pub fn can_mega_evo(
        species_name: &str,
        species_other_formes: Option<&[String]>,
        item_mega_evolves: Option<&str>,
        item_mega_stone: Option<&str>,
        base_moves: &[ID],
        item_is_z_move: bool,
        gen: u8,
    ) -> Option<String> {
        // Check Mega Rayquaza (requires Dragon Ascent)
        if let Some(other_formes) = species_other_formes {
            if let Some(first_forme) = other_formes.first() {
                if first_forme.ends_with("-Mega") {
                    // Check if it requires a move (like Rayquaza)
                    let required_move = ID::new("dragonascent");
                    if base_moves.contains(&required_move) && !item_is_z_move {
                        return Some(first_forme.clone());
                    }
                }
            }
        }

        // Check item-based mega evolution
        if let (Some(mega_evolves), Some(mega_stone)) = (item_mega_evolves, item_mega_stone) {
            // Check if item's mega evolves matches species
            if mega_evolves == species_name && mega_stone != species_name {
                return Some(mega_stone.to_string());
            }
        }

        None
    }

    /// Check if Pokemon can Ultra Burst
    /// Equivalent to canUltraBurst in battle-actions.ts
    pub fn can_ultra_burst(species_name: &str, item_id: &str) -> Option<String> {
        if (species_name == "Necrozma-Dawn-Wings" || species_name == "Necrozma-Dusk-Mane")
            && item_id == "ultranecroziumz"
        {
            return Some("Necrozma-Ultra".to_string());
        }
        None
    }

    /// Check if Pokemon can Terastallize
    /// Equivalent to canTerastallize in battle-actions.ts
    pub fn can_terastallize(
        item_is_z_move: bool,
        can_mega_evo: bool,
        gen: u8,
        tera_type: Option<&str>,
    ) -> Option<String> {
        if item_is_z_move || can_mega_evo || gen != 9 {
            return None;
        }
        tera_type.map(|t| t.to_string())
    }
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
    fn test_stat_with_boost() {
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 0), 100);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 1), 150);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 2), 200);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, -1), 66);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, 6), 400);
        assert_eq!(BattleActions::calculate_stat_with_boost(100, -6), 25);
    }

    #[test]
    fn test_recoil_damage() {
        // Chloroblast uses 50% HP
        assert_eq!(BattleActions::calc_recoil_damage(100, "chloroblast", None, 200), 100);

        // Normal recoil (1/4)
        assert_eq!(BattleActions::calc_recoil_damage(100, "doubleedge", Some((1, 4)), 200), 25);

        // No recoil
        assert_eq!(BattleActions::calc_recoil_damage(100, "tackle", None, 200), 0);
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
