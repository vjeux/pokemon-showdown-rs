//! Assist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     const moves = [];
///     for (const pokemon of target.side.pokemon) {
///         if (pokemon === target) continue;
///         for (const moveSlot of pokemon.moveSlots) {
///             const moveid = moveSlot.id;
///             const move = this.dex.moves.get(moveid);
///             if (move.flags['noassist'] || move.isZ || move.isMax) {
///                 continue;
///             }
///             moves.push(moveid);
///         }
///     }
///     let randomMove = '';
///     if (moves.length) randomMove = this.sample(moves);
///     if (!randomMove) {
///         return false;
///     }
///     this.actions.useMove(randomMove, target);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

