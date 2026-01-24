//! Futuremove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    debug_elog!("[FUTUREMOVE::ON_START] ENTRY: pokemon_pos={:?}, turn={}", pokemon_pos, battle.turn);

    // this.effectState.targetSlot = target.getSlot();
    let (target_position, target_side_index) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.position, pokemon_pos.0)
    };

    // this.effectState.endingTurn = (this.turn - 1) + 2;
    let ending_turn = (battle.turn - 1) + 2;

    // Store in slot condition's effectState
    if let Some(side) = battle.sides.get_mut(target_side_index) {
        if let Some(slot_conditions) = side.slot_conditions.get_mut(target_position) {
            if let Some(condition) = slot_conditions.get_mut(&ID::from("futuremove")) {
                // Note: targetSlot is not stored as it's not used and there's no typed field for it
                // The target position is already tracked via the slot condition's position
                condition.borrow_mut().ending_turn = Some(ending_turn);
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
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] ENTRY: pokemon_pos={:?}, turn={}", pokemon_pos, battle.turn);

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
            .map(|condition| condition.borrow().ending_turn.unwrap_or(0) as usize)
            .unwrap_or(0);

        (ending_turn, position, side_index)
    };

    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] ending_turn={}, battle.turn={}, overflowed_turn={}, should_execute={}",
        ending_turn, battle.turn, battle.turn - 1, (battle.turn - 1) >= ending_turn as i32);

    // Check if it's time to execute the future move
    // In JavaScript: if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
    // getOverflowedTurnCount() returns (this.turn - 1) for gen 8+
    if (battle.turn - 1) < ending_turn as i32 {
        debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Returning early, not time yet");
        return EventResult::Continue;
    }

    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Time to execute! Extracting data before removing condition...");

    // Extract the move data FIRST before removing the condition
    let (move_id_str, source_pos_data, move_data) = {
        if let Some(side) = battle.sides.get(target_side_index) {
            if let Some(slot_conditions) = side.slot_conditions.get(target_position) {
                if let Some(condition) = slot_conditions.get(&ID::from("futuremove")) {
                    let borrowed = condition.borrow();
                    let move_id = borrowed.move_id.clone().unwrap_or_default();
                    let source = borrowed.source;
                    let move_data = borrowed.move_data.clone();
                    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Extracted move='{}', source={:?}, move_data={:?}", move_id, source, move_data);
                    (move_id, source, move_data)
                } else {
                    (String::new(), None, None)
                }
            } else {
                (String::new(), None, None)
            }
        } else {
            (String::new(), None, None)
        }
    };

    // Remove the condition BEFORE executing on_end
    // This ensures that when the future move's onTry is called, the condition doesn't exist
    // Note: For futuremove, we intentionally call the side method directly to avoid
    // triggering the End event here since on_end_with_data handles the move execution
    if let Some(side) = battle.sides.get_mut(target_side_index) {
        debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Removing slot condition before calling on_end");
        side.remove_slot_condition(target_position, &ID::from("futuremove"));
        debug_elog!("[FUTUREMOVE::ON_RESIDUAL] remove_slot_condition returned");
    } else {
        debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Failed to get side {}", target_side_index);
    }

    // Now call on_end to execute the future move
    // The condition is removed, so onTry won't fail due to condition already existing
    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Calling on_end to execute future move");
    on_end_with_data(battle, pokemon_pos, &move_id_str, source_pos_data, move_data);

    debug_elog!("[FUTUREMOVE::ON_RESIDUAL] Returning Continue");
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
    source_pos_data: Option<(usize, usize)>,
    move_data: Option<std::collections::HashMap<String, serde_json::Value>>,
) -> EventResult {
    debug_elog!("[FUTUREMOVE::ON_END] ENTRY: pokemon_pos={:?}, turn={}, move={}, source={:?}, move_data={:?}",
        pokemon_pos, battle.turn, move_id_str, source_pos_data, move_data);

    // const data = this.effectState;
    // Data has been pre-extracted and passed as parameters
    let move_id = ID::from(move_id_str);
    let source_pos = source_pos_data;

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
    debug_elog!("[FUTUREMOVE::ON_END] About to execute move: move_id={:?}, source_pos={:?}, target_pos={:?}",
        move_id, source_pos, pokemon_pos);
    if let Some(src) = source_pos {
        debug_elog!("[FUTUREMOVE::ON_END] Calling try_spread_move_hit");
        // Set flag to indicate we're executing a future move
        // This prevents the move's onTry from queuing another future move
        battle.executing_future_move = true;

        // Create an ActiveMove from the stored move ID
        let mut active_move = match battle.dex.get_active_move(move_id.as_str()) {
            Some(am) => am,
            None => {
                debug_elog!("[FUTUREMOVE::ON_END] Failed to get ActiveMove for move_id={}", move_id);
                battle.executing_future_move = false;
                return EventResult::Continue;
            }
        };

        // Apply moveData overrides from stored condition
        // JavaScript: const hitMove = new this.dex.Move(data.moveData) as ActiveMove;
        // We need to apply the stored moveData fields, especially ignoreImmunity
        if let Some(ref move_data_map) = move_data {
            // Apply ignoreImmunity from moveData
            if let Some(ignore_immunity_value) = move_data_map.get("ignoreImmunity") {
                debug_elog!("[FUTUREMOVE::ON_END] Found ignoreImmunity in moveData: {:?}", ignore_immunity_value);
                match ignore_immunity_value {
                    serde_json::Value::Bool(false) => {
                        // ignoreImmunity: false means DO NOT ignore immunity
                        // Set to None so type immunity is checked
                        active_move.ignore_immunity = None;
                        debug_elog!("[FUTUREMOVE::ON_END] Set ignore_immunity = None (respect type immunity)");
                    },
                    serde_json::Value::Bool(true) => {
                        // ignoreImmunity: true means ignore all immunities
                        active_move.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::All);
                        debug_elog!("[FUTUREMOVE::ON_END] Set ignore_immunity = All");
                    },
                    serde_json::Value::Object(map) => {
                        // ignoreImmunity: { Type: true, ... } means ignore specific type immunities
                        let mut type_map = std::collections::HashMap::new();
                        for (key, value) in map {
                            if let serde_json::Value::Bool(b) = value {
                                type_map.insert(key.clone(), *b);
                            }
                        }
                        active_move.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::Specific(type_map));
                        debug_elog!("[FUTUREMOVE::ON_END] Set ignore_immunity = Specific");
                    },
                    _ => {
                        debug_elog!("[FUTUREMOVE::ON_END] Unexpected ignoreImmunity value type");
                    }
                }
            }
        }

        // Call trySpreadMoveHit directly, not use_move
        let targets = vec![pokemon_pos];

        // Save last_move before executing future move
        // Future moves should NOT update battle.last_move since they are delayed damage,
        // not player actions. Copycat should copy the last actual move used, not future move damage.
        let saved_last_move = battle.last_move.clone();

        let _result = crate::battle_actions::try_spread_move_hit(
            battle,
            &targets,
            src,
            &mut active_move,
            true, // notActive=true means it will call setActiveMove
        );
        // Clear the flag
        battle.executing_future_move = false;
        debug_elog!("[FUTUREMOVE::ON_END] try_spread_move_hit returned");

        // if (data.source.isActive && data.source.hasItem('lifeorb') && this.gen >= 5) {
        //     this.singleEvent('AfterMoveSecondarySelf', data.source.getItem(), data.source.itemState, data.source, target, data.source.getItem());
        // }
        let (source_is_active, source_item_id) = {
            if let Some(source_pokemon) = battle.pokemon_at(src.0, src.1) {
                (source_pokemon.is_active, source_pokemon.item.clone())
            } else {
                (false, ID::from(""))
            }
        };

        if source_is_active && source_item_id.as_str() == "lifeorb" && battle.gen >= 5 {
            debug_elog!("[FUTUREMOVE::ON_END] Source has Life Orb and is active, calling AfterMoveSecondarySelf");
            // this.singleEvent('AfterMoveSecondarySelf', data.source.getItem(), data.source.itemState, data.source, target, data.source.getItem());
            // handle_item_event will get active_move from battle.active_move (still set from try_spread_move_hit)
            battle.single_event(
                "AfterMoveSecondarySelf",
                &crate::battle::Effect::item(source_item_id),
                None,
                Some(src), // target = source of Future Sight (the user, who holds Life Orb)
                Some(pokemon_pos), // source = target of Future Sight
                Some(&crate::battle::Effect::item(ID::from("lifeorb"))),
                None,
            );
        }

        // JavaScript explicitly sets this.activeMove = null; AFTER the Life Orb event
        // This prevents clearActiveMove from updating last_move
        battle.active_move = None;

        // Restore last_move to what it was before the future move executed
        battle.last_move = saved_last_move;
    } else {
        debug_elog!("[FUTUREMOVE::ON_END] No source position, cannot execute move");
    }

    debug_elog!("[FUTUREMOVE::ON_END] Returning Continue");

    // this.checkWin();
    // JavaScript calls checkWin after executing the future move hit
    // This is critical for ending the battle early if a side loses
    battle.check_win(None);

    EventResult::Continue
}

/// onEnd wrapper for the dispatcher
/// This is called when remove_slot_condition is called elsewhere
/// It extracts data and delegates to on_end_with_data
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    debug_elog!("[FUTUREMOVE::ON_END_WRAPPER] Extracting data from condition");

    // Extract data from the condition before it's removed
    let (move_id_str, source_pos_data, move_data) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let side_index = pokemon_pos.0;
        let position = pokemon.position;

        if let Some(side) = battle.sides.get(side_index) {
            if let Some(slot_conditions) = side.slot_conditions.get(position) {
                if let Some(condition) = slot_conditions.get(&ID::from("futuremove")) {
                    let borrowed = condition.borrow();
                    let move_id = borrowed.move_id.clone().unwrap_or_default();
                    let source = borrowed.source;
                    let move_data = borrowed.move_data.clone();
                    (move_id, source, move_data)
                } else {
                    (String::new(), None, None)
                }
            } else {
                (String::new(), None, None)
            }
        } else {
            (String::new(), None, None)
        }
    };

    on_end_with_data(battle, pokemon_pos, &move_id_str, source_pos_data, move_data)
}
