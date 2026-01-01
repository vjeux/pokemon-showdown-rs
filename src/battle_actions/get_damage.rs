//! BattleActions::getDamage - Get damage for a move
//!
//! 1:1 port of getDamage from battle-actions.ts

use crate::*;

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
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    move_id: &ID,
) -> Option<i32> {
    // Get move data
    let move_data = match battle.dex.moves().get(move_id.as_str()) {
        Some(m) => m.clone(),
        None => return None,
    };

    // Check immunity first
    // JavaScript: if (!target.runImmunity(move, !suppressMessages))
    // For now, we'll do a basic type check (full immunity checking would be more complex)
    let (target_side, target_poke) = target_pos;
    let target_types = if let Some(side) = battle.sides.get(target_side) {
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
        let target_hp = if let Some(side) = battle.sides.get(target_side) {
            if let Some(pokemon) = side.pokemon.get(target_poke) {
                if battle.gen == 3 {
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

    // JavaScript: if (move.basePowerCallback) { basePower = move.basePowerCallback.call(this.battle, source, target, move); }
    // Call basePowerCallback FIRST, before any checks
    // This is for moves like Punishment that calculate their own base power
    if base_power == 0 {
        use crate::data::move_callbacks;
        if let crate::event::EventResult::Number(bp) = move_callbacks::dispatch_base_power_callback(
            battle,
            move_data.id.as_str(),
            source_pos,
            Some(target_pos),
        ) {
            base_power = bp;
            eprintln!("[GET_DAMAGE] basePowerCallback set basePower to {}", base_power);
        }
    }

    // JavaScript: if (!basePower) return basePower === 0 ? void 0 : basePower;
    // CRITICAL: This check must happen BEFORE the crit check!
    // If basePower is 0, return early (undefined for success, or the value for failure)
    if base_power == 0 {
        // For moves like Punishment, basePowerCallback would have set power from 0
        // If it's still 0 after the callback, the move deals no damage
        eprintln!("[GET_DAMAGE] basePower is still 0 after basePowerCallback, returning Some(0)");
        return Some(0); // No damage dealt, move continues
    }

    // Calculate critical hit only if we might deal damage
    // JavaScript: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    let mut crit_ratio = move_data.crit_ratio;
    let mut is_crit = false;

    // Only calculate crit if basePower is non-zero
    // This prevents PRNG calls for non-damaging moves
    if base_power > 0 {
        // Trigger ModifyCritRatio event to allow abilities to modify crit ratio
        if let Some(modified_crit) = battle.run_event(
            "ModifyCritRatio",
            Some(source_pos),
            Some(target_pos),
            Some(&move_data.id),
            Some(crit_ratio),
        ) {
            crit_ratio = modified_crit;
        }

        // Clamp crit ratio based on generation
        let crit_mult = if battle.gen <= 5 {
            crit_ratio = crit_ratio.clamp(0, 5);
            [0, 16, 8, 4, 3, 2]
        } else if battle.gen == 6 {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 16, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        } else {
            crit_ratio = crit_ratio.clamp(0, 4);
            [0, 24, 8, 2, 1, 0] // Padded to size 6, last element never accessed
        };

        // Determine if this is a critical hit
        // JavaScript: moveHit.crit = move.willCrit || false; if (move.willCrit === undefined && critRatio) moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);

        // Check if will_crit is explicitly set in active_move
        if let Some(ref active_move) = battle.active_move {
            if let Some(will_crit) = active_move.will_crit {
                is_crit = will_crit;
            } else {
                // will_crit is None (undefined), so roll for crit
                if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
                    let crit_chance = crit_mult[crit_ratio as usize];
                    if crit_chance > 0 {
                        is_crit = battle.random_chance(1, crit_chance);
                    }
                }
            }
        } else {
            // No active_move, roll normally if crit_ratio > 0
            if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
                let crit_chance = crit_mult[crit_ratio as usize];
                if crit_chance > 0 {
                    is_crit = battle.random_chance(1, crit_chance);
                }
            }
        }

        // Trigger CriticalHit event to allow abilities to prevent/modify crit
        // JavaScript: if (moveHit.crit) moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
        if is_crit {
            is_crit =
                battle.run_event_bool("CriticalHit", Some(target_pos), None, Some(&move_data.id));
        }
    }

    // Trigger BasePower event to allow abilities/items/moves to modify base power
    // JavaScript: basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
    //                                                                                          ^^^^
    //                                                                                      on_effect=true
    // When on_effect is true, the move's onBasePower handler is called (e.g., Knock Off's 1.5x boost)
    eprintln!("[GET_DAMAGE] basePower BEFORE BasePower event: {}", base_power);
    if let Some(modified_bp) = battle.run_event_with_effect(
        "BasePower",
        Some(source_pos),
        Some(target_pos),
        Some(&move_data.id),
        Some(base_power),
    ) {
        base_power = modified_bp;
        eprintln!("[GET_DAMAGE] basePower AFTER BasePower event: {}", base_power);
    } else {
        eprintln!("[GET_DAMAGE] No BasePower event modification");
    }

    // Check if base power is still 0 after BasePower event
    // For moves like Punishment, the BasePower event sets the power from 0 to the actual value
    if base_power == 0 {
        return Some(0); // No damage dealt, move continues
    }

    // Get attacker level
    let level = if let Some(side) = battle.sides.get(source_pos.0) {
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
    let mut attack = if let Some(side) = battle.sides.get(source_pos.0) {
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
    let mut defense = if let Some(side) = battle.sides.get(target_pos.0) {
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

    // JavaScript: attack = this.battle.runEvent('ModifyAtk', source, target, move, attack);
    // JavaScript: defense = this.battle.runEvent('ModifyDef', target, source, move, defense);
    // Apply stat modifier events
    if is_physical {
        // Debug: log pokemon info
        if let Some(pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
            eprintln!("[GET_DAMAGE] ModifyAtk for Pokemon: {}, Ability: {}, attack={}",
                pokemon.name, pokemon.ability, attack);
        }
        eprintln!("[GET_DAMAGE] BEFORE ModifyAtk: attack={}", attack);
        attack = battle.run_event("ModifyAtk", Some(source_pos), Some(target_pos), Some(&move_id), Some(attack)).unwrap_or(attack);
        eprintln!("[GET_DAMAGE] AFTER ModifyAtk: attack={}", attack);
        defense = battle.run_event("ModifyDef", Some(target_pos), Some(source_pos), Some(&move_id), Some(defense)).unwrap_or(defense);
    } else {
        attack = battle.run_event("ModifySpA", Some(source_pos), Some(target_pos), Some(&move_id), Some(attack)).unwrap_or(attack);
        defense = battle.run_event("ModifySpD", Some(target_pos), Some(source_pos), Some(&move_id), Some(defense)).unwrap_or(defense);
    }

    // Base damage calculation
    // JavaScript: const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
    // Must truncate at each step to match JavaScript exactly
    let step1 = battle.trunc((2 * level / 5 + 2) as f64, None) as i32;
    let step2 = battle.trunc((step1 * base_power) as f64, None) as i32;
    let step3 = battle.trunc((step2 * attack) as f64, None) as i32;
    let step4 = battle.trunc(step3 as f64 / defense.max(1) as f64, None) as i32;
    let base_damage = battle.trunc(step4 as f64 / 50.0, None) as i32;

    eprintln!("[GET_DAMAGE] level={}, basePower={}, attack={}, defense={}", level, base_power, attack, defense);
    eprintln!("[GET_DAMAGE] step1={}, step2={}, step3={}, step4={}, base_damage={}", step1, step2, step3, step4, base_damage);

    // Call modifyDamage for the full calculation (pass is_crit for damage multiplier)
    let damage = crate::battle_actions::modify_damage(battle, base_damage, source_pos, target_pos, &move_data, is_crit);

    Some(damage)
}
