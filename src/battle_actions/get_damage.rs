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
use crate::battle_actions::ActiveMove;
use crate::dex_data::{StatID, BoostID};
use crate::data::move_callbacks;

/// Get damage for a move
/// Equivalent to getDamage() in battle-actions.ts:1583
///
/// JavaScript: getDamage(source, target, move, suppressMessages = false): number | undefined | null | false
/// Rust: Returns Option<i32> where None represents undefined/null/false
pub fn get_damage(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    move_: &ActiveMove,
    suppress_messages: bool,
) -> Option<i32> {
    // JavaScript handles move as string | number | ActiveMove
    // In Rust, the caller provides the ActiveMove directly

    let move_id = &move_.id;

    // JS: if (!target.runImmunity(move, !suppressMessages)) {
    // JS:     return false;
    // JS: }
    let move_type = &move_.move_type;

    if !Pokemon::run_immunity(battle, target_pos, move_type, !suppress_messages) {
        // JS: return false;
        return None;
    }

    // JS: if (move.ohko) return this.battle.gen === 3 ? target.hp : target.maxhp;
    if move_.ohko.is_some() {
        let target_hp = battle.pokemon_at(target_pos.0, target_pos.1)?.hp;
        let target_maxhp = battle.pokemon_at(target_pos.0, target_pos.1)?.maxhp;
        return Some(if battle.gen == 3 { target_hp } else { target_maxhp });
    }

    // JS: if (move.damageCallback) return move.damageCallback.call(this.battle, source, target);
    if battle.has_move_id_callback(move_id, "damageCallback") {
        let damage_callback_result = move_callbacks::dispatch_damage_callback(
            battle,
            Some(move_),
            source_pos,
            Some(target_pos),
        );
        if let EventResult::Number(dmg) = damage_callback_result {
            return Some(dmg);
        }
    }

    // JS: if (move.damage === 'level') {
    // JS:     return source.level;
    // JS: } else if (move.damage) {
    // JS:     return move.damage;
    // JS: }
    if let Some(ref damage_value) = move_.damage {
        match damage_value {
            crate::dex::MoveDamage::Level => {
                let source_level = battle.pokemon_at(source_pos.0, source_pos.1)?.level as i32;
                return Some(source_level);
            }
            crate::dex::MoveDamage::Fixed(dmg) => {
                return Some(*dmg);
            }
        }
    }

    // JS: const category = this.battle.getCategory(move);
    let category = battle.get_category(move_id);

    // JS: let basePower: number | false | null = move.basePower;
    // JS: if (move.basePowerCallback) {
    // JS:     basePower = move.basePowerCallback.call(this.battle, source, target, move);
    // JS: }
    let mut base_power = move_.base_power;
    if battle.has_move_id_callback(move_id, "basePowerCallback") {
        let base_power_callback_result = move_callbacks::dispatch_base_power_callback(
            battle,
            Some(move_),
            source_pos,
            Some(target_pos),
        );
        if let EventResult::Number(bp) = base_power_callback_result {
            base_power = bp;
        }
    }

    // JS: if (!basePower) return basePower === 0 ? undefined : basePower;
    if base_power == 0 {
        // JS: return undefined (no damage calculation for status moves)
        return None;
    }
    if base_power < 0 {
        // JS: return basePower (which would be false or null in JS)
        return None;
    }

    // JS: basePower = this.battle.clampIntRange(basePower, 1);
    base_power = battle.clamp_int_range(base_power, Some(1), None);

    // JS: let critMult;
    // JS: let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    let mut crit_ratio = move_.crit_ratio;
    let crit_ratio_result = battle.run_event(
        "ModifyCritRatio",
        Some(crate::event::EventTarget::Pokemon(source_pos)),
        Some(target_pos),
        Some(&Effect::move_(move_id.clone())),
        EventResult::Number(crit_ratio),
        false,
        false,
    );
    if let EventResult::Number(cr) = crit_ratio_result {
        crit_ratio = cr;
    }

    // JS: if (this.battle.gen <= 5) {
    // JS:     critRatio = this.battle.clampIntRange(critRatio, 0, 5);
    // JS:     critMult = [0, 16, 8, 4, 3, 2];
    // JS: } else {
    // JS:     critRatio = this.battle.clampIntRange(critRatio, 0, 4);
    // JS:     if (this.battle.gen === 6) {
    // JS:         critMult = [0, 16, 8, 2, 1];
    // JS:     } else {
    // JS:         critMult = [0, 24, 8, 2, 1];
    // JS:     }
    // JS: }
    let crit_mult: Vec<i32> = if battle.gen <= 5 {
        crit_ratio = battle.clamp_int_range(crit_ratio, Some(0), Some(5));
        vec![0, 16, 8, 4, 3, 2]
    } else {
        crit_ratio = battle.clamp_int_range(crit_ratio, Some(0), Some(4));
        if battle.gen == 6 {
            vec![0, 16, 8, 2, 1]
        } else {
            vec![0, 24, 8, 2, 1]
        }
    };

    // JS: const moveHit = target.getMoveHitData(move);
    // JS: moveHit.crit = move.willCrit || false;
    // JS: if (move.willCrit === undefined) {
    // JS:     if (critRatio) {
    // JS:         moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
    // JS:     }
    // JS: }
    let will_crit = move_.will_crit;
    let mut is_crit = will_crit.unwrap_or(false);
    if will_crit.is_none() && crit_ratio > 0 {
        is_crit = battle.random_chance(1, crit_mult[crit_ratio as usize]);
    }

    // JS: if (moveHit.crit) {
    // JS:     moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
    // JS: }
    if is_crit {
        let crit_result = battle.run_event(
            "CriticalHit",
            Some(crate::event::EventTarget::Pokemon(target_pos)),
            None,
            Some(&Effect::move_(move_id.clone())),
            EventResult::Continue,
            false,
            false,
        );
        is_crit = !matches!(crit_result, EventResult::Boolean(false));
    }

    // Store crit in move hit data
    if let Some(move_hit_data) = battle.get_move_hit_data_mut(target_pos) {
        move_hit_data.crit = is_crit;
    }

    // JS: // happens after crit calculation
    // JS: basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
    let base_power_event_result = battle.run_event(
        "BasePower",
        Some(crate::event::EventTarget::Pokemon(source_pos)),
        Some(target_pos),
        Some(&Effect::move_(move_id.clone())),
        EventResult::Number(base_power),
        true,
        false,
    );
    if let EventResult::Number(bp) = base_power_event_result {
        base_power = bp;
    }

    // JS: if (!basePower) return 0;
    if base_power == 0 {
        return Some(0);
    }

    // JS: basePower = this.battle.clampIntRange(basePower, 1);
    base_power = battle.clamp_int_range(base_power, Some(1), None);

    // JS: // Hacked Max Moves have 0 base power, even if you Dynamax
    // JS: if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
    // JS:     basePower = 0;
    // JS: }
    let source_has_dynamax = battle.pokemon_at(source_pos.0, source_pos.1)
        .map(|p| p.volatiles.contains_key(&ID::new("dynamax")))
        .unwrap_or(false);

    // In ActiveMove, is_max is bool
    // In MoveData (from base_move lookup), is_max is Option<IsMax>
    let active_is_max = move_.is_max;
    if (!source_has_dynamax && active_is_max) ||
       (active_is_max && move_.base_move.as_ref()
           .and_then(|bm| battle.dex.moves().get_by_id(bm))
           .and_then(|m| m.is_max.as_ref())
           .is_some()) {
        base_power = 0;
    }

    // JS: const dexMove = this.dex.moves.get(move.id);
    // JS: if (source.terastallized && (source.terastallized === 'Stellar' ?
    // JS:     !source.stellarBoostedTypes.includes(move.type) : source.hasType(move.type)) &&
    // JS:     basePower < 60 && dexMove.priority <= 0 && !dexMove.multihit &&
    // JS:     // Hard move.basePower check for moves like Dragon Energy that have variable BP
    // JS:     !((move.basePower === 0 || move.basePower === 150) && move.basePowerCallback)
    // JS: ) {
    // JS:     basePower = 60;
    // JS: }
    let (source_terastallized, source_stellar_boosted, source_types) = {
        let source = battle.pokemon_at(source_pos.0, source_pos.1)?;
        (
            source.terastallized.clone(),
            source.stellar_boosted_types.clone(),
            source.types.clone(),
        )
    };

    if let Some(ref tera_type) = source_terastallized {
        // Check if the move has a basePowerCallback by checking the dex data
        // This is more efficient than calling dispatch_base_power_callback
        let has_base_power_callback = battle.has_move_id_callback(&move_.id, "basePowerCallback");

        let tera_boost_applies = if tera_type == "Stellar" {
            !source_stellar_boosted.contains(&move_type.to_string())
        } else {
            source_types.iter().any(|t| t == move_type)
        };

        if tera_boost_applies &&
           base_power < 60 &&
           move_.priority <= 0 &&
           move_.multi_hit.is_none() &&
           !((move_.base_power == 0 || move_.base_power == 150) && has_base_power_callback) {
            base_power = 60;
        }
    }

    // JS: const level = source.level;
    let level = battle.pokemon_at(source_pos.0, source_pos.1)?.level as i32;

    // JS: const attacker = move.overrideOffensivePokemon === 'target' ? target : source;
    // JS: const defender = move.overrideDefensivePokemon === 'source' ? source : target;
    let attacker_pos = if move_.override_offensive_pokemon.as_ref()
        .map(|s| s.as_str()) == Some("target") {
        target_pos
    } else {
        source_pos
    };

    let defender_pos = if move_.override_defensive_pokemon.as_ref()
        .map(|s| s.as_str()) == Some("source") {
        source_pos
    } else {
        target_pos
    };

    // JS: const isPhysical = move.category === 'Physical';
    // JS: let attackStat: StatIDExceptHP = move.overrideOffensiveStat || (isPhysical ? 'atk' : 'spa');
    // JS: const defenseStat: StatIDExceptHP = move.overrideDefensiveStat || (isPhysical ? 'def' : 'spd');
    let is_physical = category == "Physical";
    let mut attack_stat = if let Some(ref override_stat) = move_.override_offensive_stat {
        match override_stat.as_str() {
            "atk" => StatID::Atk,
            "def" => StatID::Def,
            "spa" => StatID::SpA,
            "spd" => StatID::SpD,
            "spe" => StatID::Spe,
            _ => if is_physical { StatID::Atk } else { StatID::SpA },
        }
    } else {
        if is_physical { StatID::Atk } else { StatID::SpA }
    };

    let defense_stat = if let Some(ref override_stat) = move_.override_defensive_stat {
        match override_stat.as_str() {
            "atk" => StatID::Atk,
            "def" => StatID::Def,
            "spa" => StatID::SpA,
            "spd" => StatID::SpD,
            "spe" => StatID::Spe,
            _ => if is_physical { StatID::Def } else { StatID::SpD },
        }
    } else {
        if is_physical { StatID::Def } else { StatID::SpD }
    };

    // JS: const statTable = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
    // Used for event names later

    // JS: let atkBoosts = attacker.boosts[attackStat];
    // JS: let defBoosts = defender.boosts[defenseStat];
    let attacker_boost = battle.pokemon_at(attacker_pos.0, attacker_pos.1)?
        .boosts.get(stat_id_to_boost_id(attack_stat));
    let defender_boost = battle.pokemon_at(defender_pos.0, defender_pos.1)?
        .boosts.get(stat_id_to_boost_id(defense_stat));

    let mut atk_boosts = attacker_boost;
    let mut def_boosts = defender_boost;

    // JS: let ignoreNegativeOffensive = !!move.ignoreNegativeOffensive;
    // JS: let ignorePositiveDefensive = !!move.ignorePositiveDefensive;
    let mut ignore_negative_offensive = move_.ignore_negative_offensive;
    let mut ignore_positive_defensive = move_.ignore_positive_defensive;

    // JS: if (moveHit.crit) {
    // JS:     ignoreNegativeOffensive = true;
    // JS:     ignorePositiveDefensive = true;
    // JS: }
    if is_crit {
        ignore_negative_offensive = true;
        ignore_positive_defensive = true;
    }

    // JS: const ignoreOffensive = !!(move.ignoreOffensive || (ignoreNegativeOffensive && atkBoosts < 0));
    // JS: const ignoreDefensive = !!(move.ignoreDefensive || (ignorePositiveDefensive && defBoosts > 0));
    let ignore_offensive = move_.ignore_offensive || (ignore_negative_offensive && atk_boosts < 0);
    let ignore_defensive = move_.ignore_defensive || (ignore_positive_defensive && def_boosts > 0);

    // JS: if (ignoreOffensive) {
    // JS:     this.battle.debug('Negating (sp)atk boost/penalty.');
    // JS:     atkBoosts = 0;
    // JS: }
    if ignore_offensive {
        battle.debug("Negating (sp)atk boost/penalty.");
        atk_boosts = 0;
    }

    // JS: if (ignoreDefensive) {
    // JS:     this.battle.debug('Negating (sp)def boost/penalty.');
    // JS:     defBoosts = 0;
    // JS: }
    if ignore_defensive {
        battle.debug("Negating (sp)def boost/penalty.");
        def_boosts = 0;
    }

    // JS: let attack = attacker.calculateStat(attackStat, atkBoosts, 1, source);
    // JS: let defense = defender.calculateStat(defenseStat, defBoosts, 1, target);
    let mut attack = {
        let attacker = battle.pokemon_at(attacker_pos.0, attacker_pos.1)?;
        attacker.calculate_stat(battle, attack_stat, atk_boosts, 1.0, Some(source_pos))
    };
    let mut defense = {
        let defender = battle.pokemon_at(defender_pos.0, defender_pos.1)?;
        defender.calculate_stat(battle, defense_stat, def_boosts, 1.0, Some(target_pos))
    };

    // JS: attackStat = (category === 'Physical' ? 'atk' : 'spa');
    attack_stat = if category == "Physical" { StatID::Atk } else { StatID::SpA };

    // JS: // Apply Stat Modifiers
    // JS: attack = this.battle.runEvent('Modify' + statTable[attackStat], source, target, move, attack);
    // JS: defense = this.battle.runEvent('Modify' + statTable[defenseStat], target, source, move, defense);
    let attack_event_name = match attack_stat {
        StatID::Atk => "ModifyAtk",
        StatID::SpA => "ModifySpA",
        _ => "ModifyAtk",
    };
    let defense_event_name = match defense_stat {
        StatID::Def => "ModifyDef",
        StatID::SpD => "ModifySpD",
        _ => "ModifyDef",
    };

    let attack_result = battle.run_event(
        attack_event_name,
        Some(crate::event::EventTarget::Pokemon(source_pos)),
        Some(target_pos),
        Some(&Effect::move_(move_id.clone())),
        EventResult::Number(attack),
        false,
        false,
    );
    if let EventResult::Number(atk) = attack_result {
        attack = atk;
    }

    let defense_result = battle.run_event(
        defense_event_name,
        Some(crate::event::EventTarget::Pokemon(target_pos)),
        Some(source_pos),
        Some(&Effect::move_(move_id.clone())),
        EventResult::Number(defense),
        false,
        false,
    );
    if let EventResult::Number(def) = defense_result {
        defense = def;
    }

    // JS: if (this.battle.gen <= 4 && ['explosion', 'selfdestruct'].includes(move.id) && defenseStat === 'def') {
    // JS:     defense = this.battle.clampIntRange(Math.floor(defense / 2), 1);
    // JS: }
    if battle.gen <= 4 &&
       (move_id.as_str() == "explosion" || move_id.as_str() == "selfdestruct") &&
       defense_stat == StatID::Def {
        defense = battle.clamp_int_range((defense as f64 / 2.0).floor() as i32, Some(1), None);
    }

    // JS: const tr = this.battle.trunc;
    // JS: // int(int(int(2 * L / 5 + 2) * A * P / D) / 50);
    // JS: const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
    let tr = |x: f64| battle.trunc(x, None) as i32;
    let base_damage = tr(
        tr(
            tr(
                tr(2.0 * level as f64 / 5.0 + 2.0) as f64 * base_power as f64 * attack as f64
            ) as f64 / defense as f64
        ) as f64 / 50.0
    );

    // JS: // Calculate damage modifiers separately (order differs between generations)
    // JS: return this.modifyDamage(baseDamage, source, target, move, suppressMessages);
    let final_damage = crate::battle_actions::modify_damage(
        battle,
        base_damage,
        source_pos,
        target_pos,
        move_,
        is_crit,
    );

    Some(final_damage)
}

/// Helper function to convert StatID to BoostID
fn stat_id_to_boost_id(stat: StatID) -> BoostID {
    match stat {
        StatID::Atk => BoostID::Atk,
        StatID::Def => BoostID::Def,
        StatID::SpA => BoostID::SpA,
        StatID::SpD => BoostID::SpD,
        StatID::Spe => BoostID::Spe,
        StatID::HP => BoostID::Atk, // HP doesn't have a boost, shouldn't happen
    }
}
