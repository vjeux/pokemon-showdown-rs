//! Trump Card Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // Get move slot data to check PP
    // Trump Card's power depends on remaining PP
    let move_slot = pokemon.get_move_data(move_id);

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

    // TODO: battle.debug(&format!("BP: {}", bp));
    EventResult::Number(bp)
}


