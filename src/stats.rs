//! Stat Calculator
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module handles Pokemon stat calculation based on
//! base stats, IVs, EVs, nature, and level.

use crate::data::natures::{get_nature_by_name, NatureStat};
use crate::data::species::BaseStats;

/// Calculate HP stat
pub fn calc_hp(base: u32, iv: u32, ev: u32, level: u32) -> u32 {
    if base == 1 {
        // Shedinja always has 1 HP
        return 1;
    }
    ((2 * base + iv + (ev / 4)) * level / 100) + level + 10
}

/// Calculate non-HP stat (Atk, Def, SpA, SpD, Spe)
pub fn calc_stat(base: u32, iv: u32, ev: u32, level: u32, nature_multiplier: f64) -> u32 {
    // Pokemon stat formula: floor((floor(2*base + iv + floor(ev/4)) * level / 100 + 5) * nature)
    let ev_contribution = ev / 4;
    let base_stat = ((2 * base + iv + ev_contribution) * level) / 100 + 5;
    (base_stat as f64 * nature_multiplier).floor() as u32
}

/// Calculate all stats for a Pokemon
pub fn calc_all_stats(
    base_stats: &BaseStats,
    ivs: &StatSpread,
    evs: &StatSpread,
    level: u32,
    nature: Option<&str>,
) -> StatSpread {
    let nature_def = nature.and_then(get_nature_by_name);

    let get_mult = |stat: NatureStat| -> f64 {
        nature_def.map_or(1.0, |n| n.get_multiplier(stat))
    };

    StatSpread {
        hp: calc_hp(base_stats.hp, ivs.hp, evs.hp, level),
        atk: calc_stat(base_stats.atk, ivs.atk, evs.atk, level, get_mult(NatureStat::Atk)),
        def: calc_stat(base_stats.def, ivs.def, evs.def, level, get_mult(NatureStat::Def)),
        spa: calc_stat(base_stats.spa, ivs.spa, evs.spa, level, get_mult(NatureStat::SpA)),
        spd: calc_stat(base_stats.spd, ivs.spd, evs.spd, level, get_mult(NatureStat::SpD)),
        spe: calc_stat(base_stats.spe, ivs.spe, evs.spe, level, get_mult(NatureStat::Spe)),
    }
}

/// Stat spread (for IVs, EVs, or final stats)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatSpread {
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spa: u32,
    pub spd: u32,
    pub spe: u32,
}

impl StatSpread {
    pub const fn new(hp: u32, atk: u32, def: u32, spa: u32, spd: u32, spe: u32) -> Self {
        Self { hp, atk, def, spa, spd, spe }
    }

    /// Perfect IVs (31 in all stats)
    pub const fn perfect_ivs() -> Self {
        Self::new(31, 31, 31, 31, 31, 31)
    }

    /// Zero IVs
    pub const fn zero_ivs() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }

    /// Max EVs for physical attacker (252 Atk / 252 Spe / 4 HP)
    pub const fn physical_attacker_evs() -> Self {
        Self::new(4, 252, 0, 0, 0, 252)
    }

    /// Max EVs for special attacker (252 SpA / 252 Spe / 4 HP)
    pub const fn special_attacker_evs() -> Self {
        Self::new(4, 0, 0, 252, 0, 252)
    }

    /// Max EVs for defensive (252 HP / 252 Def / 4 SpD)
    pub const fn physically_defensive_evs() -> Self {
        Self::new(252, 0, 252, 0, 4, 0)
    }

    /// Max EVs for specially defensive (252 HP / 252 SpD / 4 Def)
    pub const fn specially_defensive_evs() -> Self {
        Self::new(252, 0, 4, 0, 252, 0)
    }

    /// Total of all stats
    pub fn total(&self) -> u32 {
        self.hp + self.atk + self.def + self.spa + self.spd + self.spe
    }
}

/// Calculate stats for a specific Pokemon species by name
pub fn calc_stats_for_species(
    species_name: &str,
    ivs: &StatSpread,
    evs: &StatSpread,
    level: u32,
    nature: Option<&str>,
) -> Option<StatSpread> {
    use crate::data::species::get_species_by_name;

    let species = get_species_by_name(species_name)?;
    Some(calc_all_stats(&species.base_stats, ivs, evs, level, nature))
}

/// Get the boost multiplier for a stat stage (-6 to +6)
pub fn get_boost_multiplier(stage: i8) -> f64 {
    match stage {
        -6 => 2.0 / 8.0,
        -5 => 2.0 / 7.0,
        -4 => 2.0 / 6.0,
        -3 => 2.0 / 5.0,
        -2 => 2.0 / 4.0,
        -1 => 2.0 / 3.0,
        0 => 1.0,
        1 => 3.0 / 2.0,
        2 => 4.0 / 2.0,
        3 => 5.0 / 2.0,
        4 => 6.0 / 2.0,
        5 => 7.0 / 2.0,
        6 => 8.0 / 2.0,
        _ => 1.0,
    }
}

/// Get the accuracy/evasion boost multiplier (different formula)
pub fn get_accuracy_boost_multiplier(stage: i8) -> f64 {
    match stage {
        -6 => 3.0 / 9.0,
        -5 => 3.0 / 8.0,
        -4 => 3.0 / 7.0,
        -3 => 3.0 / 6.0,
        -2 => 3.0 / 5.0,
        -1 => 3.0 / 4.0,
        0 => 1.0,
        1 => 4.0 / 3.0,
        2 => 5.0 / 3.0,
        3 => 6.0 / 3.0,
        4 => 7.0 / 3.0,
        5 => 8.0 / 3.0,
        6 => 9.0 / 3.0,
        _ => 1.0,
    }
}

/// Apply boost to a stat
pub fn apply_boost(stat: u32, stage: i8) -> u32 {
    (stat as f64 * get_boost_multiplier(stage)).floor() as u32
}

/// Calculate the speed tier of a Pokemon
pub fn get_speed_tier(base_speed: u32, positive_nature: bool, evs: u32, level: u32) -> u32 {
    let nature_mult = if positive_nature { 1.1 } else { 1.0 };
    calc_stat(base_speed, 31, evs, level, nature_mult)
}

/// Common speed tiers at level 100 with max investment
pub fn common_speed_benchmarks() -> Vec<(&'static str, u32)> {
    vec![
        ("Base 50 neutral", 218),
        ("Base 60 neutral", 240),
        ("Base 70 neutral", 262),
        ("Base 80 neutral", 284),
        ("Base 90 neutral", 306),
        ("Base 100 neutral", 328),
        ("Base 100 +nature", 359),
        ("Base 110 neutral", 350),
        ("Base 120 neutral", 372),
        ("Base 130 neutral", 394),
        ("Base 130 +nature", 433),
        ("Base 140 neutral", 416),
        ("Base 150 neutral", 438),
        ("Regieleki (200)", 548),
        ("Regieleki +nature", 602),
    ]
}

/// Calculate damage range
pub fn calc_damage_range(
    attacker_stat: u32,
    defender_stat: u32,
    base_power: u32,
    level: u32,
    stab: bool,
    type_effectiveness: f64,
    critical: bool,
    other_modifiers: f64,
) -> (u32, u32) {
    let level_factor = (2 * level / 5) + 2;
    let base_damage = ((level_factor * base_power * attacker_stat / defender_stat) / 50) + 2;

    let mut modifier = other_modifiers;

    if stab {
        modifier *= 1.5;
    }

    modifier *= type_effectiveness;

    if critical {
        modifier *= 1.5;
    }

    // Random roll is 0.85 to 1.00
    let min_damage = ((base_damage as f64 * modifier * 0.85).floor() as u32).max(1);
    let max_damage = ((base_damage as f64 * modifier * 1.00).floor() as u32).max(1);

    (min_damage, max_damage)
}

/// Calculate if a move OHKOs
pub fn calc_ohko_chance(damage_range: (u32, u32), defender_hp: u32) -> f64 {
    let (min, max) = damage_range;

    if min >= defender_hp {
        1.0 // Guaranteed OHKO
    } else if max < defender_hp {
        0.0 // Never OHKOs
    } else {
        // Calculate probability
        let range = max - min + 1;
        let ohko_rolls = max - defender_hp + 1;
        ohko_rolls as f64 / range as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_hp() {
        // Pikachu (base 35 HP) at level 100 with 31 IV and 0 EV
        let hp = calc_hp(35, 31, 0, 100);
        assert_eq!(hp, 211);

        // Blissey (base 255 HP) at level 100 with 31 IV and 252 EV
        let hp = calc_hp(255, 31, 252, 100);
        assert_eq!(hp, 714);

        // Shedinja always has 1 HP
        let hp = calc_hp(1, 31, 252, 100);
        assert_eq!(hp, 1);
    }

    #[test]
    fn test_calc_stat() {
        // Pikachu (base 90 Speed) at level 100, 31 IV, 252 EV, neutral nature
        // Formula: floor((2*90 + 31 + 63) * 100 / 100 + 5) = 279
        let spe = calc_stat(90, 31, 252, 100, 1.0);
        assert_eq!(spe, 279);

        // With positive nature (+10%)
        // 279 * 1.1 = 306.9, floor = 306
        let spe = calc_stat(90, 31, 252, 100, 1.1);
        assert_eq!(spe, 306);

        // With negative nature (-10%)
        // 279 * 0.9 = 251.1, floor = 251
        let spe = calc_stat(90, 31, 252, 100, 0.9);
        assert_eq!(spe, 251);
    }

    #[test]
    fn test_calc_all_stats() {
        let base_stats = BaseStats::new(35, 55, 40, 50, 50, 90); // Pikachu
        let ivs = StatSpread::perfect_ivs();
        let evs = StatSpread::new(0, 252, 0, 0, 4, 252);

        let stats = calc_all_stats(&base_stats, &ivs, &evs, 100, Some("Jolly"));

        // HP: floor((2*35 + 31 + 0) * 100 / 100) + 100 + 10 = 211
        assert_eq!(stats.hp, 211);
        // Speed with Jolly (+Spe): floor((2*90 + 31 + 63) * 100 / 100 + 5) * 1.1 = 306
        assert_eq!(stats.spe, 306);
        // SpA with Jolly (-SpA): floor((2*50 + 31 + 0) * 100 / 100 + 5) * 0.9 = floor(136 * 0.9) = 122
        assert_eq!(stats.spa, 122);
    }

    #[test]
    fn test_boost_multipliers() {
        assert_eq!(get_boost_multiplier(0), 1.0);
        assert_eq!(get_boost_multiplier(1), 1.5);
        assert_eq!(get_boost_multiplier(2), 2.0);
        assert_eq!(get_boost_multiplier(-1), 2.0 / 3.0);
        assert_eq!(get_boost_multiplier(6), 4.0);
    }

    #[test]
    fn test_apply_boost() {
        let base_stat = 300;
        assert_eq!(apply_boost(base_stat, 0), 300);
        assert_eq!(apply_boost(base_stat, 1), 450);
        assert_eq!(apply_boost(base_stat, 2), 600);
        assert_eq!(apply_boost(base_stat, -1), 200);
    }

    #[test]
    fn test_damage_range() {
        // Level 100 attacker with 300 Atk using 80 BP move against 250 Def
        let (min, max) = calc_damage_range(300, 250, 80, 100, true, 1.0, false, 1.0);
        assert!(min > 0);
        assert!(max >= min);
    }

    #[test]
    fn test_ohko_chance() {
        // Guaranteed OHKO
        assert_eq!(calc_ohko_chance((400, 500), 300), 1.0);

        // Never OHKOs
        assert_eq!(calc_ohko_chance((100, 150), 300), 0.0);

        // 50% chance (roughly)
        let chance = calc_ohko_chance((250, 350), 300);
        assert!(chance > 0.0 && chance < 1.0);
    }

    #[test]
    fn test_stat_spread_totals() {
        let evs = StatSpread::physical_attacker_evs();
        assert_eq!(evs.total(), 508);

        let evs = StatSpread::special_attacker_evs();
        assert_eq!(evs.total(), 508);
    }
}
