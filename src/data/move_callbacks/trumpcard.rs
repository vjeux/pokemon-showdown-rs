//! Trump Card Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// basePowerCallback(source, target, move) {
///     const callerMoveId = move.sourceEffect || move.id;
///     const moveSlot = callerMoveId === 'instruct' ? source.getMoveData(move.id) : source.getMoveData(callerMoveId);
///     let bp;
///     if (!moveSlot) {
///         bp = 40;
///     } else {
///         switch (moveSlot.pp) {
///         case 0:
///             bp = 200;
///             break;
///         case 1:
///             bp = 80;
///             break;
///         case 2:
///             bp = 60;
///             break;
///         case 3:
///             bp = 50;
///             break;
///         default:
///             bp = 40;
///             break;
///         }
///     }
///
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
/// ```
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon (user of the move)
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Determine which move to look up: sourceEffect || move.id
    // If sourceEffect is set, use its ID, otherwise use move.id
    let caller_move_id_str = active_move.source_effect.as_ref()
        .map(|e| e.id.clone())
        .unwrap_or_else(|| active_move.id.clone());

    // Determine which move slot to check
    // If caller is 'instruct', use move.id, otherwise use caller_move_id
    let move_slot_id = if caller_move_id_str.as_str() == "instruct" {
        &active_move.id
    } else {
        &caller_move_id_str
    };

    // Find the move slot
    let move_slot = pokemon.move_slots.iter().find(|s| &s.id == move_slot_id);

    // Calculate base power based on PP
    let bp = if let Some(slot) = move_slot {
        match slot.pp {
            0 => 200,
            1 => 80,
            2 => 60,
            3 => 50,
            _ => 40,
        }
    } else {
        40
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    EventResult::Number(bp)
}
