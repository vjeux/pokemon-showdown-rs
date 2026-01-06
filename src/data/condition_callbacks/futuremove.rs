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

    eprintln!("[FUTUREMOVE::ON_RESIDUAL] ending_turn={}, battle.turn={}, should_execute={}",
        ending_turn, battle.turn, battle.turn >= ending_turn as i32);

    // Check if it's time to execute the future move
    // In JavaScript: if (this.getOverflowedTurnCount() < this.effectState.endingTurn) return;
    // getOverflowedTurnCount() returns battle.turn
    if battle.turn < ending_turn as i32 {
        return EventResult::Continue;
    }

    // time's up! Remove the slot condition which will trigger onEnd
    // target.side.removeSlotCondition(this.getAtSlot(this.effectState.targetSlot), 'futuremove');
    if let Some(side) = battle.sides.get_mut(target_side_index) {
        side.remove_slot_condition(target_position, &ID::from("futuremove"));
    }

    EventResult::Continue
}

/// onEnd
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
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // const data = this.effectState;
    // Get the future move data from the condition that's being removed
    // Since we're in onEnd, the condition is being removed, so we need to get the data
    // from the side's slot_conditions before it's fully cleaned up
    let (move_id, source_pos, target_fainted) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let side_index = pokemon_pos.0;
        let position = pokemon.position;
        let fainted = pokemon.fainted;

        // Get move data from the slot condition
        let move_id = battle.sides.get(side_index)
            .and_then(|side| side.slot_conditions.get(position))
            .and_then(|slot_conditions| slot_conditions.get(&ID::from("futuremove")))
            .and_then(|condition| condition.data.get("move"))
            .and_then(|v| v.as_str())
            .map(|s| ID::from(s))
            .unwrap_or_else(|| ID::from(""));

        let source_pos: Option<(usize, usize)> = battle.sides.get(side_index)
            .and_then(|side| side.slot_conditions.get(position))
            .and_then(|slot_conditions| slot_conditions.get(&ID::from("futuremove")))
            .and_then(|condition| condition.data.get("source"))
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        (move_id, source_pos, fainted)
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
    // TODO: Need to create the move object from stored moveData and call trySpreadMoveHit
    // For now, use use_move to execute the attack
    if let Some(src) = source_pos {
        // Use the move on the target
        battle.use_move(&move_id, src, Some(pokemon_pos), None, None, None);
    }

    // TODO: Handle Life Orb damage
    // TODO: this.checkWin();

    EventResult::Continue
}
