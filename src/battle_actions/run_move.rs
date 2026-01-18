//! BattleActions::runMove - Execute a move with full pipeline
//!
//! 1:1 port of runMove from battle-actions.ts
//!
//! NOTE: runMove is used by Instruct, Pursuit, and Dancer.
//! Most other effects use useMove instead.

use crate::*;
use crate::event::EventResult;
use crate::battle::Effect;
use crate::dex::MoveData;

/// Execute a move with full pipeline
/// Equivalent to BattleActions.runMove() in battle-actions.ts
///
/// TypeScript signature:
/// runMove(moveOrMoveName: Move | string, pokemon: Pokemon, targetLoc: number, options?: {
///     sourceEffect?: Effect | null, zMove?: string, externalMove?: boolean,
///     maxMove?: string, originalTarget?: Pokemon,
/// })
pub fn run_move(
    battle: &mut Battle,
    move_data: &MoveData,
    pokemon_pos: (usize, usize),
    target_loc: i8,
    source_effect: Option<&Effect>,
    z_move: Option<String>,
    external_move: bool,
    max_move: Option<String>,
    _original_target: Option<(usize, usize)>,
    prankster_boosted: bool,
) {
    let move_id = &move_data.id;
    // Log on turns 15-17 for debugging
    if battle.turn >= 15 && battle.turn <= 17 {
        debug_elog!("[RUN_MOVE] turn={}, move={}, pokemon={:?}, external_move={}",
            battle.turn, move_id.as_str(), pokemon_pos, external_move);
    }

    debug_elog!("[RUN_MOVE] ENTRY: move={}, pokemon=({}, {}), target_loc={}, turn={}",
        move_id.as_str(), pokemon_pos.0, pokemon_pos.1, target_loc, battle.turn);
    // Gen 4 compatibility: save old active move
    // if (this.battle.gen <= 4) oldActiveMove = this.battle.activeMove;
    let old_active_move = if battle.gen <= 4 {
        battle.active_move.clone()
    } else {
        None
    };

    // pokemon.activeMoveActions++;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.active_move_actions += 1;
    }

    // Get target
    // let target = this.battle.getTarget(pokemon, maxMove || zMove || moveOrMoveName, targetLoc, originalTarget);
    // Use the move to look up target - prefer maxMove/zMove if specified, otherwise base move
    let move_for_target_id = if let Some(ref mm) = max_move {
        ID::new(mm)
    } else if let Some(ref zm) = z_move {
        ID::new(zm)
    } else {
        move_id.clone()
    };
    let target_pos = battle.get_target(pokemon_pos, &move_for_target_id, target_loc, _original_target);

    // Get base move - we already have it passed in
    // let baseMove = this.dex.getActiveMove(moveOrMoveName);
    let base_move = move_data.clone();

    // Store original priority
    // In JavaScript: let priority = baseMove.priority;
    // In JavaScript: let pranksterBoosted = false;
    //
    // Note: pranksterBoosted tracking is handled via battle.active_move.prankster_boosted
    // which is set by the Prankster ability's onModifyPriority callback.
    // The local variable in JavaScript was only used for tracking the original state,
    // but we don't need it in Rust since we access the modified state from active_move.
    let _priority = base_move.priority;

    // Check for OverrideAction event
    // if (baseMove.id !== 'struggle' && !zMove && !maxMove && !externalMove)
    if move_id.as_str() != "struggle" && z_move.is_none() && max_move.is_none() && !external_move {
        // const changedMove = this.battle.runEvent('OverrideAction', pokemon, target, baseMove);
        // JavaScript: if (changedMove && changedMove.id !== move.id) { ... }
        // For now, just run the event - full implementation would require changing the move
        battle.run_event("OverrideAction", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Continue, false, false);
    }

    // Set active move
    // this.battle.setActiveMove(move, pokemon, target);
    battle.set_active_move(Some(move_id.clone()), Some(pokemon_pos), target_pos);

    // CRITICAL: Propagate prankster_boosted flag from the move action to the active_move
    // In JavaScript, the move object carries pranksterBoosted from when it was set by Prankster's
    // onModifyPriority callback during action speed calculation. We need to restore this flag
    // since set_active_move creates a fresh active_move.
    if prankster_boosted {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.prankster_boosted = true;
        }
    }

    // Run BeforeMove event
    // const willTryMove = this.battle.runEvent('BeforeMove', pokemon, target, move);
    // Keep the full result so we can distinguish null vs false for moveThisTurnResult
    let will_try_move_result = battle.run_event(
                "BeforeMove",
                Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
        target_pos,
        Some(&crate::battle::Effect::move_(move_id.clone())),
        crate::event::EventResult::Number(1),
        false,
        false,
    );
    let will_try_move = will_try_move_result.is_truthy();

    if !will_try_move {
        // this.battle.runEvent('MoveAborted', pokemon, target, move);
        battle.run_event("MoveAborted", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Continue, false, false);

        // this.battle.clearActiveMove(true);
        battle.clear_active_move(true);

        // pokemon.moveThisTurnResult = willTryMove;
        // IMPORTANT: JavaScript sets this to the actual value (null or false),
        // which matters for moves like Temper Flare that check `=== false`.
        // If BeforeMove returned null (e.g., Sky Drop blocking), use MoveResult::Null.
        // If it returned false, use MoveResult::Failed.
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.move_this_turn_result = match will_try_move_result {
                EventResult::Null => crate::battle_actions::MoveResult::Null,
                _ => crate::battle_actions::MoveResult::Failed,
            };
        }
        return;
    }

    // Check for 'cantusetwice' flag
    // JavaScript: if (move.flags['cantusetwice'] && pokemon.lastMove?.id === move.id) {
    //     pokemon.addVolatile(move.id);
    // }
    // NOTE: This does NOT fail/return - it just adds a volatile for tracking purposes.
    // The actual disable happens at end_turn in DisableMove event.
    let has_cantusetwice_flag = base_move.flags.contains_key("cantusetwice");
    if has_cantusetwice_flag {
        let last_move_matches = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return,
            };
            pokemon.last_move.as_ref().map_or(false, |lm| lm == move_id)
        };

        if last_move_matches {
            // Add volatile with move ID - allows tracking that this move was used via Instruct/etc.
            // The volatile is removed after the move completes (with a hint message)
            Pokemon::add_volatile(battle, pokemon_pos, move_id.clone(), None, None, None, None);
        }
    }

    // Call beforeMoveCallback
    // if (move.beforeMoveCallback)
    //     move.beforeMoveCallback.call(this, pokemon, target, move);
    //
    // JavaScript: if this callback returns true, the move is cancelled
    // (e.g., Focus Punch when lostFocus is set)
    // Clone active_move to avoid borrow issues
    let active_move_for_callback = battle.active_move.clone();
    if crate::data::move_callbacks::has_before_move_callback(active_move_for_callback.as_ref()) {
        let callback_result = crate::data::move_callbacks::dispatch_before_move_callback(battle, active_move_for_callback.as_ref(), pokemon_pos);
        if let crate::event::EventResult::Boolean(true) = callback_result {
            // Move was cancelled by beforeMoveCallback
            return;
        }
    }

    // Reset lastDamage
    // pokemon.lastDamage = 0;
    {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.last_damage = 0;
        }
    }

    // Handle locked moves and PP deduction
    // if (!externalMove)
    // Make source_effect mutable so we can update it when locked
    let mut source_effect_for_use_move = source_effect.cloned();
    if !external_move {
        // Check for locked move
        // lockedMove = this.battle.runEvent('LockMove', pokemon);
        // JavaScript: if (lockedMove === true) lockedMove = false;
        // JavaScript: if (!lockedMove) { ... deduct PP ... }
        let locked_move_result = battle.run_event("LockMove", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);

        debug_elog!("[RUN_MOVE] LockMove result: {:?}, turn={}, move={}", locked_move_result, battle.turn, move_id.as_str());

        // In JavaScript, LockMove can return:
        // - undefined/false/Continue: not locked
        // - true: locked but treat as false (choice lock edge case)
        // - move ID string: locked (two-turn moves, choice lock, etc.)
        // JavaScript: if (lockedMove === true) lockedMove = false;
        let is_locked = match &locked_move_result {
            EventResult::Continue | EventResult::Boolean(false) => false,
            EventResult::Boolean(true) => false, // JavaScript: if (lockedMove === true) lockedMove = false;
            _ => true, // Any other result (move ID, etc.) means locked
        };

        // Deduct PP only if NOT locked
        // JS: if (!lockedMove) {
        //         if (!pokemon.deductPP(baseMove, null, target) && (move.id !== 'struggle')) {
        //             this.battle.add('cant', pokemon, 'nopp', move);
        //             this.battle.clearActiveMove(true);
        //             pokemon.moveThisTurnResult = false;
        //             return;
        //         }
        //     }
        let gen = battle.gen;
        if !is_locked {
            // Create ActiveMove for deduct_pp
            let active_move_for_pp = battle.dex.get_active_move(move_id.as_str());
            let pp_deducted = if let (Some(pokemon), Some(ref am)) = (battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1), &active_move_for_pp) {
                pokemon.deduct_pp(gen, am, Some(1))
            } else {
                0
            };

            // JS: if (!pokemon.deductPP(baseMove, null, target) && (move.id !== 'struggle')) {
            //         this.battle.add('cant', pokemon, 'nopp', move);
            //         this.battle.clearActiveMove(true);
            //         pokemon.moveThisTurnResult = false;
            //         return;
            //     }
            if pp_deducted == 0 && move_id.as_str() != "struggle" {
                debug_elog!("[RUN_MOVE] No PP left for {}, adding 'cant' message and returning", move_id);
                // Get pokemon ident for the 'cant' message
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return,
                    };
                    pokemon.get_slot()
                };
                let move_name = battle.dex.moves().get_by_id(&move_id)
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| move_id.to_string());

                // JS: this.battle.add('cant', pokemon, 'nopp', move);
                battle.add("cant", &[
                    crate::battle::Arg::String(pokemon_ident),
                    crate::battle::Arg::Str("nopp"),
                    crate::battle::Arg::String(format!("move: {}", move_name)),
                ]);

                // JS: this.battle.clearActiveMove(true);
                battle.clear_active_move(true);

                // JS: pokemon.moveThisTurnResult = false;
                if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    pokemon.move_this_turn_result = crate::battle_actions::MoveResult::Failed;
                }

                return;
            }
        } else {
            // Move is locked - don't deduct PP
            // JavaScript: } else { sourceEffect = this.dex.conditions.get('lockedmove'); }
            // Set sourceEffect to 'lockedmove' condition so that use_move knows this is a locked move
            // This is important for skipping Pressure PP deduction on the attack turn
            source_effect_for_use_move = Some(Effect::condition(ID::from("lockedmove")));
            debug_elog!("[RUN_MOVE] Move is locked, setting source_effect to 'lockedmove'");
        }

        // pokemon.moveUsed(move, targetLoc);
        // Note: Need to extract gen before calling move_used to avoid borrow checker issues
        // (gen already extracted above for PP deduction)
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            // Manually inline move_used to avoid double borrow
            pokemon.last_move = Some(move_id.clone());
            if gen == 2 {
                pokemon.last_move_encore = Some(move_id.clone());
            }
            // last_move_used is set in use_move_inner after ModifyType runs
            pokemon.last_move_target_loc = Some(target_loc);
            pokemon.move_this_turn = Some(move_id.clone());
        }
    }

    // Handle Z-Move
    if let Some(zmove_name) = &z_move {
        debug_elog!("[RUN_MOVE] Z-move detected: {}", zmove_name);
        // if (pokemon.illusion) {
        //     this.battle.singleEvent('End', this.dex.abilities.get('Illusion'), pokemon.abilityState, pokemon);
        // }
        // this.battle.add('-zpower', pokemon);
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return,
            };
            pokemon.get_slot()
        };
        battle.add("-zpower", &[pokemon_ident.as_str().into()]);

        // pokemon.side.zMoveUsed = true;
        battle.sides[pokemon_pos.0].z_move_used = true;
        debug_elog!("[RUN_MOVE] Set z_move_used = true for side {}", pokemon_pos.0);

        // Disable the Z-move in the pokemon's move_slots
        // Find and disable the move that was used as a Z-move
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            let zmove_id = ID::new(zmove_name);
            debug_elog!("[RUN_MOVE] Looking for move {} in {} move_slots", zmove_id, pokemon.move_slots.len());
            for move_slot in &mut pokemon.move_slots {
                debug_elog!("[RUN_MOVE]   Checking move_slot: {} vs {}", move_slot.id, zmove_id);
                if move_slot.id == zmove_id {
                    move_slot.disabled = true;
                    debug_elog!("[RUN_MOVE] Disabled Z-move: {}", zmove_id);
                    break;
                }
            }
        }
    }

    // Dancer Petal Dance hack
    // JavaScript: const noLock = externalMove && !pokemon.volatiles['lockedmove'];
    // If this is an external move (like Dancer copy) and the Pokemon doesn't already have lockedmove,
    // we need to remove any lockedmove that gets added during the move execution.
    let no_lock = external_move && {
        if let Some(p) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            !p.volatiles.contains_key(&ID::new("lockedmove"))
        } else {
            false
        }
    };

    // Call useMove
    // const moveDidSomething = this.useMove(baseMove, pokemon, { target, sourceEffect, zMove, maxMove });
    let move_did_something = crate::battle_actions::use_move(
        battle,
        move_data,
        pokemon_pos,
        target_pos,
        source_effect_for_use_move.as_ref(),
        z_move.as_deref(),
        max_move.as_deref(),
    );

    // this.battle.lastSuccessfulMoveThisTurn = moveDidSomething ? this.battle.activeMove && this.battle.activeMove.id : null;
    battle.last_successful_move_this_turn = if move_did_something {
        battle.active_move.as_ref().map(|m| m.id.clone())
    } else {
        None
    };

    // AfterMove events
    // this.battle.singleEvent('AfterMove', move, null, pokemon, target, move);
    battle.single_event("AfterMove", &crate::battle::Effect::move_(move_id.clone()), None, Some(pokemon_pos), target_pos, Some(&crate::battle::Effect::move_(move_id.clone())), None);

    // this.battle.runEvent('AfterMove', pokemon, target, move);
    battle.run_event("AfterMove", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_pos, Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Continue, false, false);

    // Handle 'cantusetwice' hint
    // JavaScript: if (move.flags['cantusetwice'] && pokemon.removeVolatile(move.id)) {
    //     this.battle.add('-hint', `Some effects can force a Pokemon to use ${move.name} again in a row.`);
    // }
    if has_cantusetwice_flag {
        let had_volatile = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return,
            };
            pokemon.volatiles.contains_key(move_id)
        };
        if had_volatile {
            Pokemon::remove_volatile(battle, pokemon_pos, move_id);
            // Show the hint
            battle.add("-hint", &[crate::battle::Arg::String(format!("Some effects can force a Pokemon to use {} again in a row.", base_move.name))]);
        }
    }

    // Handle Dancer ability
    // if (move.flags['dance'] && moveDidSomething && !move.isExternal)
    // JavaScript source (battle-actions.ts:322-348):
    //     if (move.flags['dance'] && moveDidSomething && !move.isExternal) {
    //         const dancers = [];
    //         for (const currentPoke of this.battle.getAllActive()) {
    //             if (pokemon === currentPoke) continue;
    //             if (currentPoke.hasAbility('dancer') && !currentPoke.isSemiInvulnerable()) {
    //                 dancers.push(currentPoke);
    //             }
    //         }
    //         dancers.sort(
    //             (a, b) => -(b.storedStats['spe'] - a.storedStats['spe']) || b.abilityState.effectOrder - a.abilityState.effectOrder
    //         );
    //         const targetOf1stDance = this.battle.activeTarget!;
    //         for (const dancer of dancers) {
    //             if (this.battle.faintMessages()) break;
    //             if (dancer.fainted) continue;
    //             this.battle.add('-activate', dancer, 'ability: Dancer');
    //             const dancersTarget = !targetOf1stDance.isAlly(dancer) && pokemon.isAlly(dancer) ?
    //                 targetOf1stDance :
    //                 pokemon;
    //             const dancersTargetLoc = dancer.getLocOf(dancersTarget);
    //             this.runMove(move.id, dancer, dancersTargetLoc, { sourceEffect: this.dex.abilities.get('dancer'), externalMove: true });
    //         }
    //     }
    let has_dance_flag = base_move.flags.contains_key("dance");
    if has_dance_flag && move_did_something && !external_move {
        // const dancers = [];
        // for (const currentPoke of this.battle.getAllActive())
        let mut dancers: Vec<(usize, usize)> = Vec::new();
        let all_active = battle.get_all_active(false);

        for current_poke_pos in all_active {
            // if (pokemon === currentPoke) continue;
            if current_poke_pos == pokemon_pos {
                continue;
            }

            // if (currentPoke.hasAbility('dancer') && !currentPoke.isSemiInvulnerable())
            let has_dancer_and_not_semi_invulnerable = {
                let current_poke = match battle.pokemon_at(current_poke_pos.0, current_poke_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let has_dancer = current_poke.has_ability(battle, &["dancer"]);
                let is_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, current_poke_pos);
                has_dancer && !is_semi_invulnerable
            };

            if has_dancer_and_not_semi_invulnerable {
                dancers.push(current_poke_pos);
            }
        }

        // Dancer activates in order of lowest speed stat to highest
        // Ties go to whichever Pokemon has had the ability for the least amount of time
        // JavaScript: dancers.sort((a, b) => -(b.storedStats['spe'] - a.storedStats['spe']) || b.abilityState.effectOrder - a.abilityState.effectOrder);
        dancers.sort_by(|&a_pos, &b_pos| {
            let a_speed = battle.pokemon_at(a_pos.0, a_pos.1)
                .map(|p| p.stored_stats.spe)
                .unwrap_or(0);
            let b_speed = battle.pokemon_at(b_pos.0, b_pos.1)
                .map(|p| p.stored_stats.spe)
                .unwrap_or(0);

            // JavaScript: -(b.storedStats['spe'] - a.storedStats['spe'])
            // This is equivalent to: a.storedStats['spe'] - b.storedStats['spe']
            // Which sorts in ascending order (lowest speed first)
            let speed_cmp = a_speed.cmp(&b_speed);

            if speed_cmp != std::cmp::Ordering::Equal {
                return speed_cmp;
            }

            // Tie-breaker: abilityState.effectOrder (lower is earlier)
            // JavaScript: b.abilityState.effectOrder - a.abilityState.effectOrder
            let a_effect_order = battle.pokemon_at(a_pos.0, a_pos.1)
                .map(|p| p.ability_state.effect_order)
                .unwrap_or(0);
            let b_effect_order = battle.pokemon_at(b_pos.0, b_pos.1)
                .map(|p| p.ability_state.effect_order)
                .unwrap_or(0);

            a_effect_order.cmp(&b_effect_order)
        });

        // const targetOf1stDance = this.battle.activeTarget!;
        let target_of_1st_dance = battle.active_target;

        // for (const dancer of dancers)
        for dancer_pos in dancers {
            // if (this.battle.faintMessages()) break;
            if battle.faint_messages(false, false, true) {
                break;
            }

            // if (dancer.fainted) continue;
            let dancer_fainted = battle.pokemon_at(dancer_pos.0, dancer_pos.1)
                .map(|p| p.fainted)
                .unwrap_or(true);
            if dancer_fainted {
                continue;
            }

            // this.battle.add('-activate', dancer, 'ability: Dancer');
            let dancer_ident = {
                let dancer = match battle.pokemon_at(dancer_pos.0, dancer_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                dancer.get_slot()
            };
            battle.add("-activate", &[
                crate::battle::Arg::String(dancer_ident),
                crate::battle::Arg::Str("ability: Dancer"),
            ]);

            // const dancersTarget = !targetOf1stDance.isAlly(dancer) && pokemon.isAlly(dancer) ?
            //     targetOf1stDance :
            //     pokemon;
            let dancers_target = if let Some(target_1st) = target_of_1st_dance {
                // Check: !targetOf1stDance.isAlly(dancer) && pokemon.isAlly(dancer)
                let target_not_ally_of_dancer = !battle.is_ally(target_1st, dancer_pos);
                let pokemon_is_ally_of_dancer = battle.is_ally(pokemon_pos, dancer_pos);

                if target_not_ally_of_dancer && pokemon_is_ally_of_dancer {
                    target_1st
                } else {
                    pokemon_pos
                }
            } else {
                pokemon_pos
            };

            // const dancersTargetLoc = dancer.getLocOf(dancersTarget);
            let dancers_target_loc = {
                let dancer = match battle.pokemon_at(dancer_pos.0, dancer_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let active_per_half = battle.sides[dancer_pos.0].active.len();
                dancer.get_loc_of(dancers_target.0, dancers_target.1, active_per_half)
            };

            // this.runMove(move.id, dancer, dancersTargetLoc, { sourceEffect: this.dex.abilities.get('dancer'), externalMove: true });
            let dancer_ability = Effect::ability(ID::new("dancer"));
            run_move(
                battle,
                move_data,
                dancer_pos,
                dancers_target_loc,
                Some(&dancer_ability),
                None, // z_move
                true, // external_move
                None, // max_move
                None, // original_target
                false, // prankster_boosted - Dancer copies don't get Prankster boost
            );
        }
    }

    // Dancer Petal Dance hack cleanup
    // JavaScript: if (noLock && pokemon.volatiles['lockedmove']) delete pokemon.volatiles['lockedmove'];
    // If this was an external move that didn't originally have lockedmove,
    // remove any lockedmove that was added during the move execution.
    if no_lock {
        if let Some(p) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            if p.volatiles.contains_key(&ID::new("lockedmove")) {
                Pokemon::remove_volatile(battle, pokemon_pos, &ID::new("lockedmove"));
            }
        }
    }

    // Faint messages and check win
    // this.battle.faintMessages();
    battle.faint_messages(false, false, true);

    // this.battle.checkWin();
    // NOTE: In JS, faintMessages() is called with checkWin=true by default,
    // so if it determines a winner, checkWin() below will return early due to ended=true
    battle.check_win(None);

    // Gen 4 compatibility: restore old active move
    // if (this.battle.gen <= 4) {
    //     this.battle.activeMove = oldActiveMove;
    // }
    if battle.gen <= 4 {
        battle.active_move = old_active_move;
    }
}
