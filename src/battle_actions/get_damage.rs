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
use crate::event::EventResult;
use crate::battle::Effect;
use crate::dex_data::StatID;
use crate::battle_actions::{ActiveMove, IgnoreImmunity};

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
/// JavaScript: getDamage(source, target, move: ActiveMove, suppressMessages)
/// Now takes ActiveMove directly instead of move_id, matching JavaScript's pattern.
pub fn get_damage(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: &ActiveMove,
) -> Option<i32> {
    // Use the passed ActiveMove directly - matches JavaScript pattern
    let move_type = active_move.move_type.clone();

    // Check if move ignores immunity (e.g., Z-moves)
    let ignores_immunity = if let Some(ref ignore_imm) = active_move.ignore_immunity {
        match ignore_imm {
            IgnoreImmunity::All => true,
            IgnoreImmunity::Specific(map) => map.contains_key(&move_type),
            IgnoreImmunity::NoIgnore => false,
        }
    } else {
        false
    };

    if !ignores_immunity && !Pokemon::run_immunity(battle, target_pos, &move_type, true) {
        return None; // Immune
    }

    let (target_side, target_poke) = target_pos;

    // OHKO moves
    if active_move.ohko.is_some() {
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

    // JavaScript: if (move.damageCallback) return move.damageCallback.call(this.battle, source, target);
    // damageCallback provides custom damage calculation (e.g., Ruination deals 50% of target HP)
    use crate::data::move_callbacks;
    if let crate::event::EventResult::Number(custom_damage) = move_callbacks::dispatch_damage_callback(
        battle,
        Some(active_move),
        source_pos,
        Some(target_pos),
    ) {
        debug_elog!("[GET_DAMAGE] damageCallback returned {}", custom_damage);
        return Some(custom_damage);
    }

    // JavaScript: if (move.damage === 'level') { return source.level; } else if (move.damage) { return move.damage; }
    // Fixed damage moves (e.g., Seismic Toss deals damage equal to user's level)
    if let Some(ref damage_value) = active_move.damage {
        let source_level = if let Some(side) = battle.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                pokemon.level as i32
            } else {
                return None;
            }
        } else {
            return None;
        };
        match damage_value {
            crate::battle_actions::Damage::Level => {
                debug_elog!("[GET_DAMAGE] Fixed damage move (damage='level'), returning source.level={}", source_level);
                return Some(source_level);
            }
            crate::battle_actions::Damage::Fixed(damage_num) => {
                debug_elog!("[GET_DAMAGE] Fixed damage move (damage={}), returning {}", damage_num, damage_num);
                return Some(*damage_num);
            }
        }
    }

    // Use active_move.base_power directly
    debug_elog!("[GET_DAMAGE] move_id={}, active_move.base_power={}",
        active_move.id,
        active_move.base_power);
    let mut base_power = active_move.base_power;

    // JavaScript: if (move.basePowerCallback) { basePower = move.basePowerCallback.call(this.battle, source, target, move); }
    // CRITICAL: Always check for basePowerCallback, regardless of initial base_power!
    // Max/G-Max moves have non-zero base_power in move data but use callback to calculate actual damage
    let bp_result = move_callbacks::dispatch_base_power_callback(
        battle,
        Some(active_move),
        source_pos,
        Some(target_pos),
    );
    match bp_result {
        crate::event::EventResult::Number(bp) => {
            base_power = bp;
            debug_elog!("[GET_DAMAGE] basePowerCallback set basePower to {} (Number)", base_power);
        }
        crate::event::EventResult::Float(bp) => {
            // JavaScript returns floating-point for moves like Eruption with low HP
            // Clamp to at least 1 for non-zero values (JavaScript truthy check)
            base_power = if bp > 0.0 && bp < 1.0 { 1 } else { bp as i32 };
            debug_elog!("[GET_DAMAGE] basePowerCallback set basePower to {} (Float: {})", base_power, bp);
        }
        _ => {}
    }

    // JavaScript: if (!basePower) return basePower === 0 ? void 0 : basePower;
    // If basePower is 0, return undefined (None)
    // This is critical for Status moves - they should not trigger DamagingHit
    if base_power == 0 {
        debug_elog!("[GET_DAMAGE] basePower is 0, returning None (undefined - Status move)");
        return None; // undefined - Status move, no damage calculation
    }

    // Calculate critical hit
    // JavaScript: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    let mut crit_ratio = active_move.crit_ratio;
    let mut is_crit = false;

    // Trigger ModifyCritRatio event to allow abilities to modify crit ratio
    if let EventResult::Number(modified_crit) = battle.run_event(
                "ModifyCritRatio",
                Some(crate::event::EventTarget::Pokemon(source_pos)),
        Some(target_pos),
        Some(&Effect::move_(active_move.id.clone())),
        EventResult::Number(crit_ratio),
        false,
        false
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
    if let Some(will_crit) = active_move.will_crit {
        is_crit = will_crit;
    } else {
        // will_crit is None (undefined), so roll for crit
        if crit_ratio > 0 && crit_ratio < crit_mult.len() as i32 {
            let crit_chance = crit_mult[crit_ratio as usize];
            if crit_chance > 0 {
                is_crit = battle.random_chance(1, crit_chance);
                debug_elog!("[GET_DAMAGE CRIT] crit_ratio={}, crit_chance=1/{}, is_crit={}", crit_ratio, crit_chance, is_crit);
            }
        }
    }

    // Trigger CriticalHit event to allow abilities to prevent/modify crit
    // JavaScript: if (moveHit.crit) moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
    if is_crit {
        debug_elog!("[GET_DAMAGE CRIT] Critical hit confirmed before CriticalHit event");
        is_crit = battle.run_event(
                "CriticalHit",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
            None,
            Some(&Effect::move_(active_move.id.clone())),
            crate::event::EventResult::Number(1),
            false,
            false,
        ).is_truthy();
        debug_elog!("[GET_DAMAGE CRIT] Critical hit after CriticalHit event: {}", is_crit);
    }

    // Store crit in move_hit_data for other effects (e.g., Aurora Veil, Reflect, Light Screen)
    // JavaScript: moveHit.crit = ...
    if let Some(hit_data) = battle.get_move_hit_data_mut(target_pos) {
        hit_data.crit = is_crit;
        debug_elog!("[GET_DAMAGE] Stored crit={} in move_hit_data for target {:?}", is_crit, target_pos);
    }

    // Trigger BasePower event to allow abilities/items/moves to modify base power
    // JavaScript: basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
    //                                                                                          ^^^^
    //                                                                                      on_effect=true
    // When on_effect is true, the move's onBasePower handler is called (e.g., Knock Off's 1.5x boost)
    debug_elog!("[GET_DAMAGE] basePower BEFORE BasePower event: {}", base_power);
    let bp_result = battle.run_event(
                "BasePower",
                Some(crate::event::EventTarget::Pokemon(source_pos)),
        Some(target_pos),
        Some(&Effect::move_(active_move.id.clone())),
        EventResult::Number(base_power),
        true,
        false
    );
    match bp_result {
        EventResult::Number(modified_bp) => {
            base_power = modified_bp;
            debug_elog!("[GET_DAMAGE] basePower AFTER BasePower event (Number): {}", base_power);
        }
        EventResult::Float(modified_bp) => {
            // Items like Polkadot Bow return Float(basePower * 1.1) = Float(55.0)
            // In JavaScript, this float value is used directly and eventually floored
            // when used in the damage formula's integer operations.
            base_power = modified_bp.floor() as i32;
            debug_elog!("[GET_DAMAGE] basePower AFTER BasePower event (Float): {} -> {}", modified_bp, base_power);
        }
        _ => {
            debug_elog!("[GET_DAMAGE] No BasePower event modification");
        }
    }

    // JavaScript: if (!basePower) return 0;
    // If basePower is 0 after the BasePower event, return 0 (this is different from the early return above)
    if base_power == 0 {
        debug_elog!("[GET_DAMAGE] basePower is 0 after BasePower event, returning Some(0)");
        return Some(0);
    }

    // JavaScript: basePower = this.battle.clampIntRange(basePower, 1);
    base_power = base_power.max(1);
    debug_elog!("[GET_DAMAGE] basePower after clamp to min 1: {}", base_power);

    // JavaScript (lines 1653-1654): Hacked Max Moves have 0 base power, even if you Dynamax
    // if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
    //     basePower = 0;
    // }
    // CRITICAL: This check happens AFTER crit calculation, so crit PRNG calls are made even for Max moves without dynamax
    // The damage calculation continues with basePower=0, and modifyDamage applies minimum damage check (returns 1)
    if active_move.is_max.is_some() {
        let has_dynamax_volatile = if let Some(side) = battle.sides.get(source_pos.0) {
            if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                pokemon.has_volatile(&ID::new("dynamax"))
            } else {
                false
            }
        } else {
            false
        };

        // Check first condition: !source.volatiles["dynamax"] && move.isMax
        if !has_dynamax_volatile {
            if active_move.id.as_str() == "gmaxterror" {
                debug_elog!("[gmaxterror {}] Max move check: no dynamax volatile, setting basePower=0 and continuing (minimum damage will be 1)", battle.turn);
            }
            debug_elog!("[GET_DAMAGE] Max/G-Max move used without dynamax volatile, setting basePower=0 and continuing");
            base_power = 0; // Set basePower to 0, but continue calculation (minimum damage will be 1)
        } else if let Some(ref base_move_id) = active_move.base_move {
            // Check second condition: move.isMax && this.dex.moves.get(move.baseMove).isMax
            // This checks if the base move (the original move before dynamax conversion) is also a Max move
            if let Some(base_move_data) = battle.dex.moves().get(base_move_id.as_str()) {
                if base_move_data.is_max.is_some() {
                    // Hacked Max Move: the base move is itself a Max move
                    debug_elog!("[GET_DAMAGE] Hacked Max Move detected (base move is also Max move), setting basePower=0 and continuing");
                    base_power = 0; // Set basePower to 0, but continue calculation (minimum damage will be 1)
                }
            }
        }
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
    // Use category from the passed active_move (may be modified by onModifyMove callbacks)
    let category = active_move.category.clone();
    let is_physical = category == "Physical";

    // JavaScript: const attacker = move.overrideOffensivePokemon === 'target' ? target : source;
    // JavaScript: const defender = move.overrideDefensivePokemon === 'source' ? source : target;
    // Determine which Pokemon to use for attack/defense stats
    let attacker_pos = if active_move.override_offensive_pokemon.as_deref() == Some("target") {
        target_pos
    } else {
        source_pos
    };

    let defender_pos = if active_move.override_defensive_pokemon.as_deref() == Some("source") {
        source_pos
    } else {
        target_pos
    };

    // JavaScript: let attackStat: StatIDExceptHP = move.overrideOffensiveStat || (isPhysical ? 'atk' : 'spa');
    // JavaScript: const defenseStat: StatIDExceptHP = move.overrideDefensiveStat || (isPhysical ? 'def' : 'spd');
    // Determine which stats to use for damage calculation
    let attack_stat = active_move.override_offensive_stat.as_deref()
        .unwrap_or(if is_physical { "atk" } else { "spa" });
    let defense_stat = active_move.override_defensive_stat.as_deref()
        .unwrap_or(if is_physical { "def" } else { "spd" });

    debug_elog!("[GET_DAMAGE] is_physical={}, attack_stat={}, defense_stat={}", is_physical, attack_stat, defense_stat);

    // Get attacker and defender boosts based on the stat being used
    // JavaScript: let atkBoosts = attacker.boosts[attackStat];
    // JavaScript: let defBoosts = defender.boosts[defenseStat];
    let mut atk_boost = if let Some(side) = battle.sides.get(attacker_pos.0) {
        if let Some(pokemon) = side.pokemon.get(attacker_pos.1) {
            match attack_stat {
                "atk" => pokemon.boosts.atk,
                "spa" => pokemon.boosts.spa,
                "def" => pokemon.boosts.def,
                "spd" => pokemon.boosts.spd,
                "spe" => pokemon.boosts.spe,
                _ => if is_physical { pokemon.boosts.atk } else { pokemon.boosts.spa }
            }
        } else { 0 }
    } else { 0 };

    let mut def_boost = if let Some(side) = battle.sides.get(defender_pos.0) {
        if let Some(pokemon) = side.pokemon.get(defender_pos.1) {
            match defense_stat {
                "def" => pokemon.boosts.def,
                "spd" => pokemon.boosts.spd,
                "atk" => pokemon.boosts.atk,
                "spa" => pokemon.boosts.spa,
                "spe" => pokemon.boosts.spe,
                _ => if is_physical { pokemon.boosts.def } else { pokemon.boosts.spd }
            }
        } else { 0 }
    } else { 0 };

    // JavaScript: let ignoreNegativeOffensive = !!move.ignoreNegativeOffensive;
    // JavaScript: let ignorePositiveDefensive = !!move.ignorePositiveDefensive;
    // JavaScript: if (moveHit.crit) {
    // JavaScript:     ignoreNegativeOffensive = true;
    // JavaScript:     ignorePositiveDefensive = true;
    // JavaScript: }
    // TODO: Add ignore_negative_offensive and ignore_positive_defensive fields to MoveData
    // For now, hardcode to false since only used by specific moves like Chip Away
    let mut ignore_negative_offensive = false;
    let mut ignore_positive_defensive = false;

    if is_crit {
        ignore_negative_offensive = true;
        ignore_positive_defensive = true;
    }

    // JavaScript: const ignoreOffensive = !!(move.ignoreOffensive || (ignoreNegativeOffensive && atkBoosts < 0));
    // JavaScript: const ignoreDefensive = !!(move.ignoreDefensive || (ignorePositiveDefensive && defBoosts > 0));
    // Read from active_move fields (set from move data)
    let ignore_offensive = active_move.ignore_offensive || (ignore_negative_offensive && atk_boost < 0);
    let ignore_defensive = active_move.ignore_defensive || (ignore_positive_defensive && def_boost > 0);

    // JavaScript: if (ignoreOffensive) {
    // JavaScript:     this.battle.debug('Negating (sp)atk boost/penalty.');
    // JavaScript:     atkBoosts = 0;
    // JavaScript: }
    if ignore_offensive {
        debug_elog!("[GET_DAMAGE] Negating (sp)atk boost/penalty. (was {})", atk_boost);
        atk_boost = 0;
    }

    // JavaScript: if (ignoreDefensive) {
    // JavaScript:     this.battle.debug('Negating (sp)def boost/penalty.');
    // JavaScript:     defBoosts = 0;
    // JavaScript: }
    if ignore_defensive {
        debug_elog!("[GET_DAMAGE] Negating (sp)def boost/penalty. (was {})", def_boost);
        def_boost = 0;
    }

    // Get attack stat with boosts (now potentially modified for crits)
    // JavaScript: let attack = attacker.calculateStat(attackStat, atkBoosts, 1, source);
    // Convert string stat name to StatID
    let attack_stat_id = match attack_stat {
        "atk" => StatID::Atk,
        "spa" => StatID::SpA,
        "def" => StatID::Def,
        "spd" => StatID::SpD,
        "spe" => StatID::Spe,
        _ => if is_physical { StatID::Atk } else { StatID::SpA }
    };
    let attacker_name = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
        .map(|p| p.name.clone())
        .unwrap_or_default();
    let mut attack = battle.calculate_stat(attacker_pos, attack_stat_id, atk_boost as i8, 1.0, Some(source_pos));
    debug_elog!("[GET_DAMAGE] Attack calc (stat={:?}): pokemon={}, boost={}, attack={}",
        attack_stat_id, attacker_name, atk_boost, attack);
    if attack == 0 {
        return None;
    }

    // Get defense stat with boosts (now potentially modified for crits)
    // JavaScript: let defense = defender.calculateStat(defenseStat, defBoosts, 1, target);
    // Note: Wonder Room swap is handled inside calculate_stat
    let defense_stat_id = match defense_stat {
        "def" => StatID::Def,
        "spd" => StatID::SpD,
        "atk" => StatID::Atk,
        "spa" => StatID::SpA,
        "spe" => StatID::Spe,
        _ => if is_physical { StatID::Def } else { StatID::SpD }
    };
    let defender_name = battle.pokemon_at(defender_pos.0, defender_pos.1)
        .map(|p| p.name.clone())
        .unwrap_or_default();
    let mut defense = battle.calculate_stat(defender_pos, defense_stat_id, def_boost as i8, 1.0, Some(target_pos));
    debug_elog!("[GET_DAMAGE] Defense calc (stat={:?}): pokemon={}, boost={}, defense={}",
        defense_stat_id, defender_name, def_boost, defense);
    if defense == 0 {
        return None;
    }

    // JavaScript: attackStat = (category === 'Physical' ? 'atk' : 'spa');
    // JavaScript: attack = this.battle.runEvent('Modify' + statTable[attackStat], source, target, move, attack);
    // JavaScript: defense = this.battle.runEvent('Modify' + statTable[defenseStat], target, source, move, defense);
    //
    // IMPORTANT: The attack modifier event is based on category (Physical -> ModifyAtk, Special -> ModifySpA)
    // But the defense modifier event is based on the ACTUAL defenseStat (which respects overrideDefensiveStat)
    // This is important for moves like Psyshock that use the target's Defense despite being Special moves

    // Determine which modifier events to run based on JavaScript behavior
    let attack_modifier_event = if is_physical { "ModifyAtk" } else { "ModifySpA" };
    let defense_modifier_event = match defense_stat {
        "def" => "ModifyDef",
        "spd" => "ModifySpD",
        "atk" => "ModifyAtk",
        "spa" => "ModifySpA",
        "spe" => "ModifySpe",
        _ => if is_physical { "ModifyDef" } else { "ModifySpD" }
    };

    debug_elog!("[GET_DAMAGE] attack_modifier_event={}, defense_modifier_event={}", attack_modifier_event, defense_modifier_event);

    // Apply attack modifier event
    debug_elog!("[GET_DAMAGE] BEFORE {}: attack={}", attack_modifier_event, attack);
    match battle.run_event(attack_modifier_event, Some(crate::event::EventTarget::Pokemon(source_pos)), Some(target_pos), Some(&Effect::move_(active_move.id.clone())), EventResult::Number(attack), false, false) {
        EventResult::Number(n) => {
            debug_elog!("[GET_DAMAGE] AFTER {}: attack changed from {} to {}", attack_modifier_event, attack, n);
            attack = n;
        },
        EventResult::Float(multiplier) => {
            let old_attack = attack;
            attack = battle.modify_f(attack, multiplier);
            debug_elog!("[GET_DAMAGE] AFTER {}: Float multiplier {}x applied, attack changed from {} to {}", attack_modifier_event, multiplier, old_attack, attack);
        }
        _ => {
            debug_elog!("[GET_DAMAGE] AFTER {}: no change, attack={}", attack_modifier_event, attack);
        }
    }

    // Apply defense modifier event
    debug_elog!("[GET_DAMAGE] BEFORE {}: defense={}", defense_modifier_event, defense);
    match battle.run_event(defense_modifier_event, Some(crate::event::EventTarget::Pokemon(target_pos)), Some(source_pos), Some(&Effect::move_(active_move.id.clone())), EventResult::Number(defense), false, false) {
        EventResult::Number(n) => {
            debug_elog!("[GET_DAMAGE] AFTER {}: defense changed from {} to {}", defense_modifier_event, defense, n);
            defense = n;
        },
        EventResult::Float(multiplier) => {
            let old_defense = defense;
            defense = battle.modify_f(defense, multiplier);
            debug_elog!("[GET_DAMAGE] AFTER {}: Float multiplier {}x applied, defense changed from {} to {}", defense_modifier_event, multiplier, old_defense, defense);
        }
        _ => {
            debug_elog!("[GET_DAMAGE] AFTER {}: no change, defense={}", defense_modifier_event, defense);
        }
    }

    // Base damage calculation
    // JavaScript: const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
    // Must truncate at each step to match JavaScript exactly
    let step1 = battle.trunc((2 * level / 5 + 2) as f64, None) as i32;
    let step2 = battle.trunc((step1 * base_power) as f64, None) as i32;
    let step3 = battle.trunc((step2 * attack) as f64, None) as i32;
    let step4 = battle.trunc(step3 as f64 / defense.max(1) as f64, None) as i32;
    let base_damage = battle.trunc(step4 as f64 / 50.0, None) as i32;

    debug_elog!("[GET_DAMAGE] level={}, basePower={}, attack={}, defense={}", level, base_power, attack, defense);
    debug_elog!("[GET_DAMAGE] step1={}, step2={}, step3={}, step4={}, base_damage={}", step1, step2, step3, step4, base_damage);

    // Call modifyDamage for the full calculation (pass is_crit for damage multiplier)
    let damage = crate::battle_actions::modify_damage(battle, base_damage, source_pos, target_pos, active_move, is_crit);

    Some(damage)
}
