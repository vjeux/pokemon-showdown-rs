//! Sleep Talk Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return source.status === 'slp' || source.hasAbility('comatose');
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(pokemon) {
///     const moves = [];
///     for (const moveSlot of pokemon.moveSlots) {
///         const moveid = moveSlot.id;
///         if (!moveid) continue;
///         const move = this.dex.moves.get(moveid);
///         if (move.flags['nosleeptalk'] || move.flags['charge'] || (move.isZ && move.basePower !== 1) || move.isMax) {
///             continue;
///         }
///         moves.push(moveid);
///     }
///     let randomMove = '';
///     if (moves.length) randomMove = this.sample(moves);
///     if (!randomMove) {
///         return false;
///     }
///     this.actions.useMove(randomMove, pokemon);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

