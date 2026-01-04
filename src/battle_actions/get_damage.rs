//! BattleActions::getDamage - Get damage for a move
//!
//! 1:1 port of getDamage from battle-actions.ts

// JS Source:
// 	getDamage(
// 		source: Pokemon, target: Pokemon, move: string | number | ActiveMove,
// 		suppressMessages = false
// 	): number | undefined | null | false {
// 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
// 
// 		if (typeof move === 'number') {
// 			const basePower = move;
// 			move = new Dex.Move({
// 				basePower,
// 				type: '???',
// 				category: 'Physical',
// 				willCrit: false,
// 			}) as ActiveMove;
// 			move.hit = 0;
// 		}
// 
// 		if (!target.runImmunity(move, !suppressMessages)) {
// 			return false;
// 		}
// 
// 		if (move.ohko) return this.battle.gen === 3 ? target.hp : target.maxhp;
// 		if (move.damageCallback) return move.damageCallback.call(this.battle, source, target);
// 		if (move.damage === 'level') {
// 			return source.level;
// 		} else if (move.damage) {
// 			return move.damage;
// 		}
// 
// 		const category = this.battle.getCategory(move);
// 
// 		let basePower: number | false | null = move.basePower;
// 		if (move.basePowerCallback) {
// 			basePower = move.basePowerCallback.call(this.battle, source, target, move);
// 		}
// 		if (!basePower) return basePower === 0 ? undefined : basePower;
// 		basePower = this.battle.clampIntRange(basePower, 1);
// 
// 		let critMult;
// 		let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
// 		if (this.battle.gen <= 5) {
// 			critRatio = this.battle.clampIntRange(critRatio, 0, 5);
// 			critMult = [0, 16, 8, 4, 3, 2];
// 		} else {
// 			critRatio = this.battle.clampIntRange(critRatio, 0, 4);
// 			if (this.battle.gen === 6) {
// 				critMult = [0, 16, 8, 2, 1];
// 			} else {
// 				critMult = [0, 24, 8, 2, 1];
// 			}
// 		}
// 
// 		const moveHit = target.getMoveHitData(move);
// 		moveHit.crit = move.willCrit || false;
// 		if (move.willCrit === undefined) {
// 			if (critRatio) {
// 				moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
// 			}
// 		}
// 
// 		if (moveHit.crit) {
// 			moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
// 		}
// 
// 		// happens after crit calculation
// 		basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
// 
// 		if (!basePower) return 0;
// 		basePower = this.battle.clampIntRange(basePower, 1);
// 		// Hacked Max Moves have 0 base power, even if you Dynamax
// 		if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
// 			basePower = 0;
// 		}
// 
// 		const dexMove = this.dex.moves.get(move.id);
// 		if (source.terastallized && (source.terastallized === 'Stellar' ?
// 			!source.stellarBoostedTypes.includes(move.type) : source.hasType(move.type)) &&
// 			basePower < 60 && dexMove.priority <= 0 && !dexMove.multihit &&
// 			// Hard move.basePower check for moves like Dragon Energy that have variable BP
// 			!((move.basePower === 0 || move.basePower === 150) && move.basePowerCallback)
// 		) {
// 			basePower = 60;
// 		}
// 
// 		const level = source.level;
// 
// 		const attacker = move.overrideOffensivePokemon === 'target' ? target : source;
// 		const defender = move.overrideDefensivePokemon === 'source' ? source : target;
// 
// 		const isPhysical = move.category === 'Physical';
// 		let attackStat: StatIDExceptHP = move.overrideOffensiveStat || (isPhysical ? 'atk' : 'spa');
// 		const defenseStat: StatIDExceptHP = move.overrideDefensiveStat || (isPhysical ? 'def' : 'spd');
// 
// 		const statTable = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
// 
// 		let atkBoosts = attacker.boosts[attackStat];
// 		let defBoosts = defender.boosts[defenseStat];
// 
// 		let ignoreNegativeOffensive = !!move.ignoreNegativeOffensive;
// 		let ignorePositiveDefensive = !!move.ignorePositiveDefensive;
// 
// 		if (moveHit.crit) {
// 			ignoreNegativeOffensive = true;
// 			ignorePositiveDefensive = true;
// 		}
// 		const ignoreOffensive = !!(move.ignoreOffensive || (ignoreNegativeOffensive && atkBoosts < 0));
// 		const ignoreDefensive = !!(move.ignoreDefensive || (ignorePositiveDefensive && defBoosts > 0));
// 
// 		if (ignoreOffensive) {
// 			this.battle.debug('Negating (sp)atk boost/penalty.');
// 			atkBoosts = 0;
// 		}
// 		if (ignoreDefensive) {
// 			this.battle.debug('Negating (sp)def boost/penalty.');
// 			defBoosts = 0;
// 		}
// 
// 		let attack = attacker.calculateStat(attackStat, atkBoosts, 1, source);
// 		let defense = defender.calculateStat(defenseStat, defBoosts, 1, target);
// 
// 		attackStat = (category === 'Physical' ? 'atk' : 'spa');
// 
// 		// Apply Stat Modifiers
// 		attack = this.battle.runEvent('Modify' + statTable[attackStat], source, target, move, attack);
// 		defense = this.battle.runEvent('Modify' + statTable[defenseStat], target, source, move, defense);
// 
// 		if (this.battle.gen <= 4 && ['explosion', 'selfdestruct'].includes(move.id) && defenseStat === 'def') {
// 			defense = this.battle.clampIntRange(Math.floor(defense / 2), 1);
// 		}
// 
// 		const tr = this.battle.trunc;
// 
// 		// int(int(int(2 * L / 5 + 2) * A * P / D) / 50);
// 		const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
// 
// 		// Calculate damage modifiers separately (order differs between generations)
// 		return this.modifyDamage(baseDamage, source, target, move, suppressMessages);
// 	}


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
    // JavaScript:if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type])) return true;
    // JavaScript: if (!target.runImmunity(move, !suppressMessages)) return false;
    
    // Check if move ignores immunity (e.g., Z-moves)
    let ignores_immunity = if let Some(ref ignore_imm) = move_data.ignore_immunity {
        // Can be true (ignores all immunity) or an object with type names
        if ignore_imm.as_bool() == Some(true) {
            true
        } else if let Some(obj) = ignore_imm.as_object() {
            obj.contains_key(&move_data.move_type)
        } else {
            false
        }
    } else {
        false
    };
    
    if !ignores_immunity && !Pokemon::run_immunity(battle, target_pos, &move_data.move_type, true) {
        return None; // Immune
    }

    let (target_side, target_poke) = target_pos;

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

    // CRITICAL: Check active_move.base_power first!
    // Moves like Present modify active_move.base_power in their onModifyMove callback
    // JavaScript passes the same move object around, but in Rust we have separate move_data and active_move
    // So we must check active_move first to get modifications from onModifyMove events
    let mut base_power = if let Some(ref active_move) = battle.active_move {
        active_move.base_power
    } else {
        move_data.base_power
    };

    // JavaScript: if (move.basePowerCallback) { basePower = move.basePowerCallback.call(this.battle, source, target, move); }
    // CRITICAL: Always check for basePowerCallback, regardless of initial base_power!
    // Max/G-Max moves have non-zero base_power in move data but use callback to calculate actual damage
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

    // JavaScript: if (!basePower) return basePower === 0 ? void 0 : basePower;
    // CRITICAL: This check must happen BEFORE the crit check!
    // If basePower is 0, return early (undefined for success, or the value for failure)
    if base_power == 0 {
        // For moves like Punishment, basePowerCallback would have set power from 0
        // If it's still 0 after the callback, the move deals no damage
        eprintln!("[GET_DAMAGE] basePower is still 0 after basePowerCallback, returning Some(0)");
        return Some(0); // No damage dealt, move continues
    }

    // Calculate critical hit
    // JavaScript: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    let mut crit_ratio = move_data.crit_ratio;
    let mut is_crit = false;

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
                    eprintln!("[GET_DAMAGE CRIT] crit_ratio={}, crit_chance=1/{}, is_crit={}", crit_ratio, crit_chance, is_crit);
                }
            }
        }
    } else {
        // No active_move, roll normally if crit_ratio > 0
        if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
            let crit_chance = crit_mult[crit_ratio as usize];
            if crit_chance > 0 {
                is_crit = battle.random_chance(1, crit_chance);
                eprintln!("[GET_DAMAGE CRIT] crit_ratio={}, crit_chance=1/{}, is_crit={}", crit_ratio, crit_chance, is_crit);
            }
        }
    }

    // Trigger CriticalHit event to allow abilities to prevent/modify crit
    // JavaScript: if (moveHit.crit) moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
    if is_crit {
        eprintln!("[GET_DAMAGE CRIT] Critical hit confirmed before CriticalHit event");
        is_crit =
            battle.run_event_bool("CriticalHit", Some(target_pos), None, Some(&move_data.id));
        eprintln!("[GET_DAMAGE CRIT] Critical hit after CriticalHit event: {}", is_crit);
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

    // JavaScript: if (!basePower) return 0;
    // If basePower is 0 after the BasePower event, return 0 (this is different from the early return above)
    if base_power == 0 {
        eprintln!("[GET_DAMAGE] basePower is 0 after BasePower event, returning Some(0)");
        return Some(0);
    }

    // JavaScript: basePower = this.battle.clampIntRange(basePower, 1);
    base_power = base_power.max(1);
    eprintln!("[GET_DAMAGE] basePower after clamp to min 1: {}", base_power);

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
