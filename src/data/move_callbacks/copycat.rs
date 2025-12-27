//! Copycat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

