//! Copycat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // let move: Move | ActiveMove | null = this.lastMove;
    // if (!move) return;
    let mut move_id = match &battle.last_move {
        Some(id) => id.clone(),
        None => {
            // return;
            return EventResult::Continue;
        }
    };

    // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    // TODO: isMax and baseMove not yet implemented
    // For now, we'll skip this check

    // if (move.flags['failcopycat'] || move.isZ || move.isMax) {
    //     return false;
    // }
    let move_data = match battle.dex.get_move_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    if move_data.flags.contains_key("failcopycat") {
        // TODO: isZ, isMax not yet implemented
        return EventResult::Boolean(false);
    }

    // this.actions.useMove(move.id, pokemon);
    // TODO: Implement use_move method in Battle
    // battle.use_move(&move_id, pokemon_pos);

    EventResult::Continue
}
