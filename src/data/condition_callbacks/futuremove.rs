//! Futuremove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target) {
///     this.effectState.targetSlot = target.getSlot();
///     this.effectState.endingTurn = (this.turn - 1) + 2;
///     if (this.effectState.endingTurn >= 254) {
///         this.hint(`In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.`);
///     }
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE::ON_START] ENTRY: pokemon_pos={:?}, turn={}", pokemon_pos, battle.turn);

    // this.effectState.targetSlot = target.getSlot();
    let (target_slot, target_position, target_side_index) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.get_slot(), pokemon.position, pokemon_pos.0)
    };

    // this.effectState.endingTurn = (this.turn - 1) + 2;
    let ending_turn = (battle.turn - 1) + 2;

    // Store in slot condition's effectState
    if let Some(side) = battle.sides.get_mut(target_side_index) {
        if let Some(slot_conditions) = side.slot_conditions.get_mut(target_position) {
            if let Some(condition) = slot_conditions.get_mut(&ID::from("futuremove")) {
                condition.data.insert(
                    "targetSlot".to_string(),
                    serde_json::Value::String(target_slot),
                );
                condition.data.insert(
                    "endingTurn".to_string(),
                    serde_json::Value::Number(ending_turn.into()),
                );
            }
        }
    }

    // if (this.effectState.endingTurn >= 254)
    if ending_turn >= 254 {
        // this.hint(`In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.`);
        battle.hint("In Gen 8+, Future attacks will never resolve when used on the 255th turn or later.", false, None);
    }

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidualOrder: 3,
/// onResidual(target: Pokemon) {
///     if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
///     target.side.removeSlotCondition(this.getAtSlot(this.effectState.targetSlot), 'futuremove');
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE::ON_RESIDUAL] ENTRY: pokemon_pos={:?}, turn={}", pokemon_pos, battle.turn);

    // if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
    // Get the ending turn from the slot condition data
    let (ending_turn, target_position, target_side_index) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let side_index = pokemon_pos.0;
        let position = pokemon.position;

        // Get endingTurn from the slot condition
        let ending_turn = battle.sides.get(side_index)
            .and_then(|side| side.slot_conditions.get(position))
            .and_then(|slot_conditions| slot_conditions.get(&ID::from("futuremove")))
            .and_then(|condition| condition.data.get("endingTurn"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as usize;

        (ending_turn, position, side_index)
    };

    eprintln!("[FUTUREMOVE::ON_RESIDUAL] ending_turn={}, battle.turn={}, overflowed_turn={}, should_execute={}",
        ending_turn, battle.turn, battle.turn - 1, (battle.turn - 1) >= ending_turn as i32);

    // Check if it's time to execute the future move
    // In JavaScript: if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
    // getOverflowedTurnCount() returns (this.turn - 1) for gen 8+
    if (battle.turn - 1) < ending_turn as i32 {
        eprintln!("[FUTUREMOVE::ON_RESIDUAL] Returning early, not time yet");
        return EventResult::Continue;
    }

    eprintln!("[FUTUREMOVE::ON_RESIDUAL] Time to execute! Extracting data before removing condition...");

    // Extract the move data FIRST before removing the condition
    let (move_id_str, source_pos_data) = {
        if let Some(side) = battle.sides.get(target_side_index) {
            if let Some(slot_conditions) = side.slot_conditions.get(target_position) {
                if let Some(condition) = slot_conditions.get(&ID::from("futuremove")) {
                    let move_id = condition.data.get("move")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let source = condition.data.get("source").cloned();
                    eprintln!("[FUTUREMOVE::ON_RESIDUAL] Extracted move='{}', source={:?}", move_id, source);
                    (move_id, source)
                } else {
                    (String::new(), None)
                }
            } else {
                (String::new(), None)
            }
        } else {
            (String::new(), None)
        }
    };

    // Remove the condition BEFORE executing on_end
    // This ensures that when the future move's onTry is called, the condition doesn't exist
    if let Some(side) = battle.sides.get_mut(target_side_index) {
        eprintln!("[FUTUREMOVE::ON_RESIDUAL] Removing slot condition before calling on_end");
        side.remove_slot_condition(target_position, &ID::from("futuremove"));
        eprintln!("[FUTUREMOVE::ON_RESIDUAL] remove_slot_condition returned");
    } else {
        eprintln!("[FUTUREMOVE::ON_RESIDUAL] Failed to get side {}", target_side_index);
    }

    // Now call on_end to execute the future move
    // The condition is removed, so onTry won't fail due to condition already existing
    eprintln!("[FUTUREMOVE::ON_RESIDUAL] Calling on_end to execute future move");
    on_end_with_data(battle, pokemon_pos, &move_id_str, source_pos_data);

    eprintln!("[FUTUREMOVE::ON_RESIDUAL] Returning Continue");
    EventResult::Continue
}

/// onEnd (with pre-extracted data)
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(target) {
///     const data = this.effectState;
///     // time's up; time to hit! :D
///     const move = this.dex.moves.get(data.move);
///     if (target.fainted || target === data.source) {
///         this.hint(`${move.name} did not hit because the target is ${(target.fainted ? 'fainted' : 'the user')}.`);
///         return;
///     }
///
///     this.add('-end', target, 'move: ' + move.name);
///     target.removeVolatile('Protect');
///     target.removeVolatile('Endure');
///
///     if (data.source.hasAbility('infiltrator') && this.gen >= 6) {
///         data.moveData.infiltrates = true;
///     }
///     if (data.source.hasAbility('normalize') && this.gen >= 6) {
///         data.moveData.type = 'Normal';
///     }
///     const hitMove = new this.dex.Move(data.moveData) as ActiveMove;
///
///     this.actions.trySpreadMoveHit([target], data.source, hitMove, true);
///     if (data.source.isActive && data.source.hasItem('lifeorb') && this.gen >= 5) {
///         this.singleEvent('AfterMoveSecondarySelf', data.source.getItem(), data.source.itemState, data.source, target, data.source.getItem());
///     }
///     this.activeMove = null;
///
///     this.checkWin();
/// }
/// ```
pub fn on_end_with_data(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    move_id_str: &str,
    source_pos_data: Option<serde_json::Value>,
) -> EventResult {
    eprintln!("[FUTUREMOVE::ON_END] ENTRY: pokemon_pos={:?}, turn={}, move={}, source={:?}",
        pokemon_pos, battle.turn, move_id_str, source_pos_data);

    // const data = this.effectState;
    // Data has been pre-extracted and passed as parameters
    let move_id = ID::from(move_id_str);
    let source_pos: Option<(usize, usize)> = source_pos_data
        .and_then(|v| serde_json::from_value(v).ok());

    let target_fainted = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.fainted
    };

    // if (target.fainted || target === data.source)
    if target_fainted {
        // this.hint(`${move.name} did not hit because the target is fainted.`);
        return EventResult::Continue;
    }

    // Check if target is the source
    if let Some(src) = source_pos {
        if src == pokemon_pos {
            // this.hint(`${move.name} did not hit because the target is the user.`);
            return EventResult::Continue;
        }
    }

    // this.add('-end', target, 'move: ' + move.name);
    let move_name = battle.dex.moves().get(move_id.as_str())
        .map(|m| m.name.clone())
        .unwrap_or_else(|| move_id.to_string());

    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-end", &[
        crate::battle::Arg::String(target_ident),
        crate::battle::Arg::String(format!("move: {}", move_name)),
    ]);

    // target.removeVolatile('Protect');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("protect"));

    // target.removeVolatile('Endure');
    crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("endure"));

    // Execute the future move hit
    // this.actions.trySpreadMoveHit([target], data.source, hitMove, true);
    // JavaScript creates a new Move object from stored moveData and calls trySpreadMoveHit directly
    // For now, call trySpreadMoveHit with the move_id and not_active=true
    eprintln!("[FUTUREMOVE::ON_END] About to execute move: move_id={:?}, source_pos={:?}, target_pos={:?}",
        move_id, source_pos, pokemon_pos);
    if let Some(src) = source_pos {
        eprintln!("[FUTUREMOVE::ON_END] Calling try_spread_move_hit");
        // Set flag to indicate we're executing a future move
        // This prevents the move's onTry from queuing another future move
        battle.executing_future_move = true;
        // Call trySpreadMoveHit directly, not use_move
        let targets = vec![pokemon_pos];
        let _result = crate::battle_actions::try_spread_move_hit(
            battle,
            &targets,
            src,
            &move_id,
            true, // notActive=true means it will call setActiveMove
        );
        // Clear the flag
        battle.executing_future_move = false;
        eprintln!("[FUTUREMOVE::ON_END] try_spread_move_hit returned");
    } else {
        eprintln!("[FUTUREMOVE::ON_END] No source position, cannot execute move");
    }

    eprintln!("[FUTUREMOVE::ON_END] Returning Continue");

    // TODO: Handle Life Orb damage
    // TODO: this.checkWin();

    EventResult::Continue
}

/// onEnd wrapper for the dispatcher
/// This is called when remove_slot_condition is called elsewhere
/// It extracts data and delegates to on_end_with_data
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE::ON_END_WRAPPER] Extracting data from condition");

    // Extract data from the condition before it's removed
    let (move_id_str, source_pos_data) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let side_index = pokemon_pos.0;
        let position = pokemon.position;

        if let Some(side) = battle.sides.get(side_index) {
            if let Some(slot_conditions) = side.slot_conditions.get(position) {
                if let Some(condition) = slot_conditions.get(&ID::from("futuremove")) {
                    let move_id = condition.data.get("move")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let source = condition.data.get("source").cloned();
                    (move_id, source)
                } else {
                    (String::new(), None)
                }
            } else {
                (String::new(), None)
            }
        } else {
            (String::new(), None)
        }
    };

    on_end_with_data(battle, pokemon_pos, &move_id_str, source_pos_data)
}
