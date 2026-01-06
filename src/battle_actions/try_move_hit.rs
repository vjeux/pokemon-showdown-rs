//! BattleActions::tryMoveHit - Try to hit with a field-wide move
//!
//! 1:1 port of tryMoveHit from battle-actions.ts

// JS Source:
// 	tryMoveHit(targetOrTargets: Pokemon | Pokemon[], pokemon: Pokemon, move: ActiveMove): number | undefined | false | '' {
// 		const target = Array.isArray(targetOrTargets) ? targetOrTargets[0] : targetOrTargets;
// 		const targets = Array.isArray(targetOrTargets) ? targetOrTargets : [target];
//
// 		this.battle.setActiveMove(move, pokemon, targets[0]);
//
// 		let hitResult = this.battle.singleEvent('Try', move, null, pokemon, target, move) &&
// 			this.battle.singleEvent('PrepareHit', move, {}, target, pokemon, move) &&
// 			this.battle.runEvent('PrepareHit', pokemon, target, move);
// 		if (!hitResult) {
// 			if (hitResult === false) {
// 				this.battle.add('-fail', pokemon);
// 				this.battle.attrLastMove('[still]');
// 			}
// 			return false;
// 		}
//
// 		const isFFAHazard = move.target === 'foeSide' && this.battle.gameType === 'freeforall';
// 		if (move.target === 'all') {
// 			hitResult = this.battle.runEvent('TryHitField', target, pokemon, move);
// 		} else if (isFFAHazard) {
// 			const hitResults: any[] = this.battle.runEvent('TryHitSide', targets, pokemon, move);
// 			// if some side blocked the move, prevent the move from executing against any other sides
// 			if (hitResults.some(result => !result)) return false;
// 			hitResult = true;
// 		} else {
// 			hitResult = this.battle.runEvent('TryHitSide', target, pokemon, move);
// 		}
// 		if (!hitResult) {
// 			if (hitResult === false) {
// 				this.battle.add('-fail', pokemon);
// 				this.battle.attrLastMove('[still]');
// 			}
// 			return false;
// 		}
// 		return this.moveHit(isFFAHazard ? targets : target, pokemon, move);
// 	}

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::move_hit::move_hit;
use crate::battle_actions::DamageResult;

/// Try to hit with a field-wide move (all, foeSide, allySide, allyTeam targets)
/// Equivalent to tryMoveHit() in battle-actions.ts
///
/// JavaScript signature:
/// tryMoveHit(targetOrTargets: Pokemon | Pokemon[], pokemon: Pokemon, move: ActiveMove): number | undefined | false | ''
pub fn try_move_hit(
    battle: &mut Battle,
    target_or_targets: &[(usize, usize)],
    pokemon_pos: (usize, usize),
    move_id: &ID,
) -> DamageResult {
    // const target = Array.isArray(targetOrTargets) ? targetOrTargets[0] : targetOrTargets;
    // const targets = Array.isArray(targetOrTargets) ? targetOrTargets : [target];
    let target = target_or_targets[0];
    let targets = target_or_targets;

    // this.battle.setActiveMove(move, pokemon, targets[0]);
    battle.set_active_move(Some(move_id.clone()), Some(pokemon_pos), Some(targets[0]));

    // let hitResult = this.battle.singleEvent('Try', move, null, pokemon, target, move) &&
    //     this.battle.singleEvent('PrepareHit', move, {}, target, pokemon, move) &&
    //     this.battle.runEvent('PrepareHit', pokemon, target, move);
    let try_result = battle.single_event(
        "Try",
        move_id,
        Some(pokemon_pos),
        Some(target),
        Some(move_id),
        None,
    );

    let prepare_hit_single = battle.single_event(
        "PrepareHit",
        move_id,
        Some(target),
        Some(pokemon_pos),
        Some(move_id),
        None,
    );

    let prepare_hit_run = battle.run_event_bool(
        "PrepareHit",
        Some(pokemon_pos),
        Some(target),
        Some(move_id),
    );

    // JavaScript uses && operator which short-circuits on false
    let mut hit_result = !matches!(try_result, EventResult::Boolean(false) | EventResult::NotFail | EventResult::Null);
    hit_result = hit_result && !matches!(prepare_hit_single, EventResult::Boolean(false) | EventResult::NotFail | EventResult::Null);
    hit_result = hit_result && prepare_hit_run;

    // if (!hitResult) {
    //     if (hitResult === false) {
    //         this.battle.add('-fail', pokemon);
    //         this.battle.attrLastMove('[still]');
    //     }
    //     return false;
    // }
    if !hit_result {
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return DamageResult::Failed,
            };
            format!("p{}a: {}", pokemon_pos.0 + 1, pokemon.set.species)
        };
        battle.add("-fail", &[crate::battle::Arg::String(pokemon_ident)]);
        battle.attr_last_move(&["[still]"]);
        return DamageResult::Failed;
    }

    // Get move target type from active_move
    let move_target = battle.active_move.as_ref()
        .map(|m| m.target.clone())
        .unwrap_or_else(|| "normal".to_string());

    // const isFFAHazard = move.target === 'foeSide' && this.battle.gameType === 'freeforall';
    let is_ffa_hazard = move_target == "foeSide" && battle.game_type == crate::dex_data::GameType::FreeForAll;

    // if (move.target === 'all') {
    //     hitResult = this.battle.runEvent('TryHitField', target, pokemon, move);
    // } else if (isFFAHazard) {
    //     const hitResults: any[] = this.battle.runEvent('TryHitSide', targets, pokemon, move);
    //     if (hitResults.some(result => !result)) return false;
    //     hitResult = true;
    // } else {
    //     hitResult = this.battle.runEvent('TryHitSide', target, pokemon, move);
    // }
    hit_result = if move_target == "all" {
        battle.run_event_bool("TryHitField", Some(target), Some(pokemon_pos), Some(move_id))
    } else if is_ffa_hazard {
        // For FFA hazards, check all targets
        // JavaScript: hitResults.some(result => !result)
        // If any target fails, return false
        let mut all_success = true;
        for &target_pos in targets {
            let result = battle.run_event_bool("TryHitSide", Some(target_pos), Some(pokemon_pos), Some(move_id));
            if !result {
                all_success = false;
                break;
            }
        }
        if !all_success {
            return DamageResult::Failed;
        }
        true
    } else {
        battle.run_event_bool("TryHitSide", Some(target), Some(pokemon_pos), Some(move_id))
    };

    // if (!hitResult) {
    //     if (hitResult === false) {
    //         this.battle.add('-fail', pokemon);
    //         this.battle.attrLastMove('[still]');
    //     }
    //     return false;
    // }
    if !hit_result {
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return DamageResult::Failed,
            };
            format!("p{}a: {}", pokemon_pos.0 + 1, pokemon.set.species)
        };
        battle.add("-fail", &[crate::battle::Arg::String(pokemon_ident)]);
        battle.attr_last_move(&["[still]"]);
        return DamageResult::Failed;
    }

    // return this.moveHit(isFFAHazard ? targets : target, pokemon, move);
    // Call moveHit with either all targets (for FFA hazards) or just the first target
    if is_ffa_hazard {
        // For FFA hazards, pass all targets
        let target_opts: Vec<Option<(usize, usize)>> = targets.iter().map(|&t| Some(t)).collect();
        move_hit(battle, &target_opts, pokemon_pos, move_id, None, false, false)
    } else {
        // For single target, pass just the first target
        let target_opts = vec![Some(target)];
        move_hit(battle, &target_opts, pokemon_pos, move_id, None, false, false)
    }
}
