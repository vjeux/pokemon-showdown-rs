//! Battle Actions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Handles all battle actions: moves, switches, damage calculation, etc.

use crate::dex::{Dex, MoveData, Accuracy};
use crate::pokemon::Pokemon;
use crate::dex_data::{ID, BoostsTable};

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
#[derive(Debug, Clone, Default)]
pub struct MoveHitData {
    pub crit: bool,
    pub type_mod: f64,
    pub damage: u32,
}

/// Battle action handler
pub struct BattleActions<'a> {
    pub dex: &'a Dex,
    pub gen: u8,
}

impl<'a> BattleActions<'a> {
    pub fn new(dex: &'a Dex, gen: u8) -> Self {
        Self { dex, gen }
    }

    /// Check if a move hits based on accuracy
    pub fn accuracy_check(
        &self,
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &MoveData,
        random_value: u32,  // Random value in [0, 100)
    ) -> bool {
        match move_data.accuracy {
            Accuracy::AlwaysHits => true,
            Accuracy::Percent(acc) => {
                if acc == 0 {
                    return false;
                }

                // Apply accuracy/evasion modifiers
                let mut accuracy = acc as i32;

                // Get accuracy boost
                let acc_boost = attacker.boosts.accuracy as i32;
                if acc_boost > 0 {
                    accuracy = accuracy * (3 + acc_boost) / 3;
                } else if acc_boost < 0 {
                    accuracy = accuracy * 3 / (3 - acc_boost);
                }

                // Get evasion boost
                let eva_boost = defender.boosts.evasion as i32;
                if eva_boost > 0 {
                    accuracy = accuracy * 3 / (3 + eva_boost);
                } else if eva_boost < 0 {
                    accuracy = accuracy * (3 - eva_boost) / 3;
                }

                // Clamp to valid range
                let accuracy = accuracy.clamp(0, 100) as u32;

                random_value < accuracy
            }
        }
    }

    /// Check if a move scores a critical hit
    pub fn crit_check(
        &self,
        _attacker: &Pokemon,
        _move_data: &MoveData,
        random_value: u32,  // Random value
    ) -> bool {
        // Base crit ratio (from move's critRatio field if any)
        let crit_ratio = 1; // Default crit ratio is 1

        // In Gen 7+, crit ratios: 0=1/24, 1=1/8, 2=1/2, 3+=always
        let crit_chance = match self.gen {
            1..=5 => {
                // Gen 1-5 crit chances
                match crit_ratio {
                    0 => 0,
                    1 => 16,  // 1/16
                    2 => 8,   // 1/8
                    3 => 4,   // 1/4
                    4 => 3,   // 1/3
                    _ => 2,   // 1/2
                }
            }
            6 => {
                // Gen 6 crit chances
                match crit_ratio {
                    0 => 0,
                    1 => 16,
                    2 => 8,
                    3 => 2,
                    _ => 1,
                }
            }
            _ => {
                // Gen 7+ crit chances
                match crit_ratio {
                    0 => 0,
                    1 => 24,
                    2 => 8,
                    3 => 2,
                    _ => 1,
                }
            }
        };

        if crit_chance == 0 {
            return false;
        }

        random_value % crit_chance == 0
    }

    /// Get type effectiveness multiplier
    pub fn get_type_effectiveness(&self, move_type: &str, defender_types: &[String]) -> f64 {
        self.dex.get_type_effectiveness(move_type, defender_types)
    }

    /// Calculate the stat modifier from boost stages
    /// Returns the multiplier as a fraction (numerator, denominator)
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

    /// Main damage calculation function
    ///
    /// Implements the standard Pokemon damage formula:
    /// damage = ((2L/5 + 2) * P * A/D) / 50 + 2
    ///
    /// Then applies modifiers for:
    /// - STAB (Same Type Attack Bonus)
    /// - Type effectiveness
    /// - Critical hits
    /// - Random factor (85-100%)
    pub fn calculate_damage(
        &self,
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &MoveData,
        is_crit: bool,
        random_factor: u32,  // Should be in range [85, 100]
    ) -> DamageResult {
        // Status moves don't deal damage
        if move_data.category == "Status" {
            return DamageResult::NoDamage;
        }

        let base_power = move_data.base_power;
        if base_power == 0 {
            return DamageResult::NoDamage;
        }

        // Get defender's types for immunity check
        let defender_types = defender.types.clone();

        // Check immunity
        let type_effectiveness = self.get_type_effectiveness(&move_data.move_type, &defender_types);
        if type_effectiveness == 0.0 {
            return DamageResult::Immune;
        }

        // Determine if physical or special
        let is_physical = move_data.category == "Physical";

        // Get attack and defense stats
        let (attack_stat, defense_stat) = if is_physical {
            (attacker.stored_stats.atk as u32, defender.stored_stats.def as u32)
        } else {
            (attacker.stored_stats.spa as u32, defender.stored_stats.spd as u32)
        };

        // Get boosts
        let (atk_boost, def_boost) = if is_physical {
            (attacker.boosts.atk, defender.boosts.def)
        } else {
            (attacker.boosts.spa, defender.boosts.spd)
        };

        // Apply boosts (crits ignore negative atk boosts and positive def boosts)
        let effective_atk_boost = if is_crit && atk_boost < 0 { 0 } else { atk_boost };
        let effective_def_boost = if is_crit && def_boost > 0 { 0 } else { def_boost };

        let attack = Self::calculate_stat_with_boost(attack_stat, effective_atk_boost);
        let defense = Self::calculate_stat_with_boost(defense_stat, effective_def_boost);

        // Prevent division by zero
        let defense = defense.max(1);

        // Level
        let level = attacker.level as u32;

        // Base damage calculation: ((2L/5 + 2) * P * A/D) / 50 + 2
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense) / 50 + 2;

        // Apply critical hit modifier
        let damage = if is_crit {
            let crit_mult = if self.gen >= 6 { 1.5 } else { 2.0 };
            (base_damage as f64 * crit_mult) as u32
        } else {
            base_damage
        };

        // Apply random factor (85-100%)
        let random_factor = random_factor.clamp(85, 100);
        let damage = damage * random_factor / 100;

        // Apply STAB (Same Type Attack Bonus)
        let has_type = attacker.types.iter().any(|t| t == &move_data.move_type);
        let stab = if has_type {
            1.5
        } else {
            1.0
        };
        let damage = (damage as f64 * stab) as u32;

        // Apply type effectiveness
        let damage = (damage as f64 * type_effectiveness) as u32;

        // Minimum damage is 1
        let damage = damage.max(1);

        DamageResult::Damage(damage)
    }

    /// Apply damage to a Pokemon
    pub fn apply_damage(defender: &mut Pokemon, damage: u32) -> u32 {
        let actual_damage = damage.min(defender.hp);
        defender.hp = defender.hp.saturating_sub(damage);
        actual_damage
    }

    /// Apply stat boosts to a Pokemon
    pub fn apply_boosts(pokemon: &mut Pokemon, boosts: &BoostsTable) -> bool {
        let mut any_changed = false;

        let old_atk = pokemon.boosts.atk;
        pokemon.boosts.atk = (pokemon.boosts.atk + boosts.atk).clamp(-6, 6);
        if pokemon.boosts.atk != old_atk {
            any_changed = true;
        }

        let old_def = pokemon.boosts.def;
        pokemon.boosts.def = (pokemon.boosts.def + boosts.def).clamp(-6, 6);
        if pokemon.boosts.def != old_def {
            any_changed = true;
        }

        let old_spa = pokemon.boosts.spa;
        pokemon.boosts.spa = (pokemon.boosts.spa + boosts.spa).clamp(-6, 6);
        if pokemon.boosts.spa != old_spa {
            any_changed = true;
        }

        let old_spd = pokemon.boosts.spd;
        pokemon.boosts.spd = (pokemon.boosts.spd + boosts.spd).clamp(-6, 6);
        if pokemon.boosts.spd != old_spd {
            any_changed = true;
        }

        let old_spe = pokemon.boosts.spe;
        pokemon.boosts.spe = (pokemon.boosts.spe + boosts.spe).clamp(-6, 6);
        if pokemon.boosts.spe != old_spe {
            any_changed = true;
        }

        any_changed
    }

    /// Apply a status condition
    pub fn apply_status(&self, pokemon: &mut Pokemon, status: &str) -> bool {
        if !pokemon.status.is_empty() {
            return false;  // Already has a status
        }

        // Helper to check type
        let has_type = |type_name: &str| pokemon.types.iter().any(|t| t == type_name);

        // Check type-based immunities
        match status {
            "brn" => {
                if has_type("Fire") {
                    return false;
                }
            }
            "par" => {
                if self.gen >= 6 && has_type("Electric") {
                    return false;
                }
            }
            "psn" | "tox" => {
                if has_type("Poison") || has_type("Steel") {
                    return false;
                }
            }
            "frz" => {
                if has_type("Ice") {
                    return false;
                }
            }
            _ => {}
        }

        pokemon.set_status(ID::new(status));
        true
    }

    /// Check if a Pokemon can move (not fully paralyzed, frozen, asleep, etc.)
    pub fn can_move(&self, pokemon: &Pokemon, random_value: u32) -> bool {
        let status = pokemon.status.as_str();
        match status {
            "slp" => {
                // Check if sleep counter allows waking up
                pokemon.status_state.duration.map(|t| t == 0).unwrap_or(false)
            }
            "frz" => {
                // 20% chance to thaw in Gen 2+
                if self.gen >= 2 {
                    random_value % 5 == 0
                } else {
                    false
                }
            }
            "par" => {
                // 25% chance of full paralysis
                random_value % 4 != 0
            }
            "fnt" => false,
            "" => true,
            _ => true,
        }
    }

    /// Calculate speed for turn order
    pub fn get_effective_speed(&self, pokemon: &Pokemon) -> u32 {
        let base_speed = pokemon.stored_stats.spe as u32;
        let boosted_speed = Self::calculate_stat_with_boost(base_speed, pokemon.boosts.spe);

        // Apply paralysis speed reduction
        if pokemon.status.as_str() == "par" {
            boosted_speed / 2
        } else {
            boosted_speed
        }
    }

    /// Compare speeds for turn order (returns true if a goes before b)
    pub fn speed_compare(
        &self,
        a: &Pokemon,
        b: &Pokemon,
        a_priority: i8,
        b_priority: i8,
        speed_tie_random: bool,
    ) -> bool {
        // Higher priority goes first
        if a_priority != b_priority {
            return a_priority > b_priority;
        }

        // Otherwise compare speed
        let a_speed = self.get_effective_speed(a);
        let b_speed = self.get_effective_speed(b);

        if a_speed != b_speed {
            return a_speed > b_speed;
        }

        // Speed tie - random
        speed_tie_random
    }

    /// Process end-of-turn effects (burn damage, poison damage, leftovers, etc.)
    pub fn process_residual(&self, pokemon: &mut Pokemon) -> Vec<(String, i32)> {
        let mut effects = Vec::new();

        if pokemon.is_fainted() {
            return effects;
        }

        // Status damage
        let status = pokemon.status.as_str().to_string();
        match status.as_str() {
            "brn" => {
                // Burn does 1/16 (Gen 7+) or 1/8 (before) of max HP
                let damage = if self.gen >= 7 {
                    pokemon.maxhp / 16
                } else {
                    pokemon.maxhp / 8
                }.max(1);
                pokemon.hp = pokemon.hp.saturating_sub(damage);
                effects.push(("burn_damage".to_string(), damage as i32));
            }
            "psn" => {
                // Regular poison does 1/8 of max HP
                let damage = (pokemon.maxhp / 8).max(1);
                pokemon.hp = pokemon.hp.saturating_sub(damage);
                effects.push(("poison_damage".to_string(), damage as i32));
            }
            "tox" => {
                // Toxic damage increases each turn
                let toxic_counter = pokemon.status_state.duration.unwrap_or(1);
                let damage = (pokemon.maxhp * toxic_counter / 16).max(1);
                pokemon.hp = pokemon.hp.saturating_sub(damage);
                effects.push(("toxic_damage".to_string(), damage as i32));

                // Increment toxic counter
                pokemon.status_state.duration = Some(pokemon.status_state.duration.unwrap_or(1) + 1);
            }
            "slp" => {
                // Decrement sleep counter
                if let Some(duration) = pokemon.status_state.duration {
                    if duration > 0 {
                        pokemon.status_state.duration = Some(duration - 1);
                    }
                    if duration <= 1 {
                        pokemon.status = ID::empty();
                        effects.push(("wake_up".to_string(), 0));
                    }
                }
            }
            _ => {}
        }

        effects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pokemon::PokemonSet;

    fn create_test_pokemon(name: &str, level: u8, hp: u32) -> Pokemon {
        let set = PokemonSet {
            name: name.to_string(),
            species: name.to_string(),
            level,
            ..Default::default()
        };
        let mut pokemon = Pokemon::new(&set, 0, 0);
        pokemon.hp = hp;
        pokemon.maxhp = hp;
        pokemon.stored_stats.atk = 100;
        pokemon.stored_stats.def = 100;
        pokemon.stored_stats.spa = 100;
        pokemon.stored_stats.spd = 100;
        pokemon.stored_stats.spe = 100;
        pokemon.types = vec!["Normal".to_string()];
        pokemon
    }

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
    fn test_damage_calculation() {
        let dex = Dex::load_default().unwrap();
        let actions = BattleActions::new(&dex, 9);

        let attacker = create_test_pokemon("Pikachu", 50, 100);
        let mut defender = create_test_pokemon("Squirtle", 50, 100);
        defender.types = vec!["Water".to_string()];

        let thunderbolt = dex.get_move("Thunderbolt").unwrap();

        // Test with no crit, max random
        match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
            DamageResult::Damage(dmg) => {
                // Super effective (2x) against water
                assert!(dmg > 0);
            }
            _ => panic!("Expected damage"),
        }
    }

    #[test]
    fn test_immunity() {
        let dex = Dex::load_default().unwrap();
        let actions = BattleActions::new(&dex, 9);

        let attacker = create_test_pokemon("Pikachu", 50, 100);
        let mut defender = create_test_pokemon("Sandshrew", 50, 100);
        defender.types = vec!["Ground".to_string()];

        let thunderbolt = dex.get_move("Thunderbolt").unwrap();

        match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
            DamageResult::Immune => (),
            _ => panic!("Expected immunity"),
        }
    }

    #[test]
    fn test_apply_damage() {
        let mut pokemon = create_test_pokemon("Test", 50, 100);
        assert_eq!(pokemon.hp, 100);

        BattleActions::apply_damage(&mut pokemon, 30);
        assert_eq!(pokemon.hp, 70);

        BattleActions::apply_damage(&mut pokemon, 100);
        assert_eq!(pokemon.hp, 0);
        assert!(pokemon.is_fainted());
    }

    #[test]
    fn test_apply_boosts() {
        let mut pokemon = create_test_pokemon("Test", 50, 100);
        assert_eq!(pokemon.boosts.atk, 0);

        let boosts = BoostsTable { atk: 2, ..Default::default() };
        BattleActions::apply_boosts(&mut pokemon, &boosts);
        assert_eq!(pokemon.boosts.atk, 2);

        // Test clamping at +6
        let boosts = BoostsTable { atk: 10, ..Default::default() };
        BattleActions::apply_boosts(&mut pokemon, &boosts);
        assert_eq!(pokemon.boosts.atk, 6);
    }

    #[test]
    fn test_speed_comparison() {
        let dex = Dex::load_default().unwrap();
        let actions = BattleActions::new(&dex, 9);

        let mut fast = create_test_pokemon("Fast", 50, 100);
        fast.stored_stats.spe = 150;

        let mut slow = create_test_pokemon("Slow", 50, 100);
        slow.stored_stats.spe = 50;

        // Higher speed goes first
        assert!(actions.speed_compare(&fast, &slow, 0, 0, true));
        assert!(!actions.speed_compare(&slow, &fast, 0, 0, true));

        // Priority overrides speed
        assert!(actions.speed_compare(&slow, &fast, 1, 0, true));
    }
}
