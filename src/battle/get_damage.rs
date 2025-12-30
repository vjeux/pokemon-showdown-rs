use crate::*;

impl Battle {

    // =========================================================================
    // MOVE EXECUTION - Core damage and hit logic
    // Ported from battle-actions.ts
    // =========================================================================

    /// Get damage for a move
    /// Equivalent to getDamage() in battle-actions.ts:1583
    ///
    /// Returns:
    /// - Some(damage) - amount of damage to deal
    /// - Some(0) - move succeeds but deals 0 damage
    /// - None - move fails (no message)
    ///
    /// The false case (with error message) is handled by returning 0 and
    /// letting the caller add the fail message.
    pub fn get_damage(
        &mut self,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        move_id: &ID,
    ) -> Option<i32> {
        // Get move data
        let move_data = match self.dex.get_move(move_id.as_str()) {
            Some(m) => m.clone(),
            None => return None,
        };

        // Check immunity first
        // JavaScript: if (!target.runImmunity(move, !suppressMessages))
        // For now, we'll do a basic type check (full immunity checking would be more complex)
        let (target_side, target_poke) = target_pos;
        let target_types = if let Some(side) = self.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_poke) {
                pokemon.types.clone()
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Check type immunity
        let effectiveness =
            crate::data::typechart::get_effectiveness_multi(&move_data.move_type, &target_types);
        if effectiveness == 0.0 {
            return None; // Immune
        }

        // OHKO moves
        if move_data.ohko.is_some() {
            let target_hp = if let Some(side) = self.sides.get(target_side) {
                if let Some(pokemon) = side.pokemon.get(target_poke) {
                    if self.gen == 3 {
                        pokemon.hp
                    } else {
                        pokemon.maxhp
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            return Some(target_hp);
        }

        // Fixed damage moves
        if let Some(_heal_tuple) = move_data.heal {
            // Heal moves have (numerator, denominator) format
            // But damage field would be different - this is actually heal, not damage
            // For actual fixed damage, we'd check move.damage field
            // For now, skip this
        }

        let mut base_power = move_data.base_power;
        if base_power == 0 {
            return Some(0); // undefined in JS - no damage dealt, move continues
        }

        // Calculate critical hit
        // JavaScript: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
        let mut crit_ratio = move_data.crit_ratio;

        // Trigger ModifyCritRatio event to allow abilities to modify crit ratio
        if let Some(modified_crit) = self.run_event(
            "ModifyCritRatio",
            Some(source_pos),
            Some(target_pos),
            Some(&move_data.id),
            Some(crit_ratio),
        ) {
            crit_ratio = modified_crit;
        }

        // Clamp crit ratio based on generation
        let crit_mult = if self.gen <= 5 {
            crit_ratio = crit_ratio.clamp(0, 5);
            [0, 16, 8, 4, 3, 2]
        } else if self.gen == 6 {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 16, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        } else {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 24, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        };

        // Determine if this is a critical hit
        // JavaScript: moveHit.crit = move.willCrit || false; if (move.willCrit === undefined && critRatio) moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
        let mut is_crit = false;
        if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
            let crit_chance = crit_mult[crit_ratio as usize];
            if crit_chance > 0 {
                is_crit = self.random_chance(1, crit_chance);
            }
        }

        // Trigger CriticalHit event to allow abilities to prevent/modify crit
        // JavaScript: if (moveHit.crit) moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
        if is_crit {
            is_crit =
                self.run_event_bool("CriticalHit", Some(target_pos), None, Some(&move_data.id));
        }

        // Trigger BasePower event to allow abilities/items to modify base power
        // JavaScript: basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
        if let Some(modified_bp) = self.run_event(
            "BasePower",
            Some(source_pos),
            Some(target_pos),
            Some(&move_data.id),
            Some(base_power),
        ) {
            base_power = modified_bp;
        }

        // Get attacker level
        let level = if let Some(side) = self.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                pokemon.level as i32
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Determine attack and defense stats
        let is_physical = move_data.category == "Physical";

        // Get attack stat with boosts
        let attack = if let Some(side) = self.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                if is_physical {
                    let boost = pokemon.boosts.atk;
                    let base_stat = pokemon.stored_stats.atk;
                    let (num, denom) = BattleActions::get_boost_modifier(boost);
                    (base_stat * num / denom).max(1)
                } else {
                    let boost = pokemon.boosts.spa;
                    let base_stat = pokemon.stored_stats.spa;
                    let (num, denom) = BattleActions::get_boost_modifier(boost);
                    (base_stat * num / denom).max(1)
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Get defense stat with boosts
        let defense = if let Some(side) = self.sides.get(target_pos.0) {
            if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                if is_physical {
                    let boost = pokemon.boosts.def;
                    let base_stat = pokemon.stored_stats.def;
                    let (num, denom) = BattleActions::get_boost_modifier(boost);
                    (base_stat * num / denom).max(1)
                } else {
                    let boost = pokemon.boosts.spd;
                    let base_stat = pokemon.stored_stats.spd;
                    let (num, denom) = BattleActions::get_boost_modifier(boost);
                    (base_stat * num / denom).max(1)
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        // Base damage calculation
        // JavaScript: const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
        // Must truncate at each step to match JavaScript exactly
        let step1 = self.trunc((2 * level / 5 + 2) as f64);
        let step2 = self.trunc((step1 * base_power) as f64);
        let step3 = self.trunc((step2 * attack) as f64);
        let step4 = self.trunc(step3 as f64 / defense.max(1) as f64);
        let base_damage = self.trunc(step4 as f64 / 50.0);

        // Call modifyDamage for the full calculation (pass is_crit for damage multiplier)
        let damage = self.modify_damage(base_damage, source_pos, target_pos, &move_data, is_crit);

        eprintln!("DEBUG [get_damage]: move={}, source=p{}a, target=p{}a, base_damage={}, final_damage={}",
                 move_id.as_str(), source_pos.0 + 1, target_pos.0 + 1, base_damage, damage);

        Some(damage)
    }
}
