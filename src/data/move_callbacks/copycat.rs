//! Copycat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(pokemon) {
///     let move: Move | ActiveMove | null = this.lastMove;
///     if (!move) return;
///
///     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
///     if (move.flags['failcopycat'] || move.isZ || move.isMax) {
///         return false;
///     }
///     this.actions.useMove(move.id, pokemon);
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // let move: Move | ActiveMove | null = this.lastMove;
    // if (!move) return;
    let move_id = match &battle.last_move {
        Some(active_move) => active_move.id.clone(),
        None => {
            // return;
            return EventResult::Continue;
        }
    };

    // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    let actual_move_id = {
        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        if move_data.is_max.is_some() {
            if let Some(ref base_move) = move_data.base_move {
                base_move.clone()
            } else {
                move_id.clone()
            }
        } else {
            move_id.clone()
        }
    };

    // if (move.flags['failcopycat'] || move.isZ || move.isMax) {
    //     return false;
    // }
    let move_data = match battle.dex.moves().get_by_id(&actual_move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    if move_data.flags.contains_key("failcopycat")
        || move_data.is_z.is_some()
        || move_data.is_max.is_some()
    {
        return EventResult::Boolean(false);
    }

    // this.actions.useMove(move.id, pokemon);
    // TODO: Implement use_move method in Battle
    // battle.use_move(&move_id, pokemon_pos);

    EventResult::Continue
}
