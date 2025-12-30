use crate::*;
use crate::dex::MoveData;
use crate::data::typechart::get_effectiveness_multi;

impl<'a> BattleActions<'a> {

    /// Calculate damage for a move
    /// This is a simplified damage calculation for testing purposes.
    /// The full damage calculation is in getDamage in battle-actions.ts
    pub fn calculate_damage(
        &self,
        attacker: &Pokemon,
        defender: &Pokemon,
        move_data: &MoveData,
        is_crit: bool,
        random_factor: i32,
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
            let base_atk = attacker.stored_stats.spa;
            let base_def = defender.stored_stats.spd;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        } else {
            let atk_boost = attacker.boosts.atk;
            let def_boost = defender.boosts.def;
            let base_atk = attacker.stored_stats.atk;
            let base_def = defender.stored_stats.def;
            (
                Self::calculate_stat_with_boost(base_atk, atk_boost),
                Self::calculate_stat_with_boost(base_def, def_boost),
            )
        };

        // Basic damage formula: ((2 * Level / 5 + 2) * Power * A/D) / 50 + 2
        let level = attacker.level as i32;
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense.max(1)) / 50 + 2;

        // Apply STAB (Same Type Attack Bonus)
        let stab = if attacker.types.iter().any(|t| t == &move_data.move_type) {
            1.5
        } else {
            1.0
        };

        // Apply type effectiveness
        let damage = (base_damage as f64 * stab * effectiveness) as i32;

        // Apply critical hit (1.5x in Gen 6+)
        let damage = if is_crit {
            (damage as f64 * 1.5) as i32
        } else {
            damage
        };

        // Apply random factor (0.85 to 1.0, passed in as 85-100)
        let damage = damage * random_factor / 100;

        DamageResult::Damage(damage.max(1))
    }
}
