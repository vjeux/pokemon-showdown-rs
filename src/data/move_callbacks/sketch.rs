//! Sketch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     const move = target.lastMove;
///     if (source.transformed || !move || source.moves.includes(move.id)) return false;
///     if (move.flags['nosketch'] || move.isZ || move.isMax) return false;
///     const sketchIndex = source.moves.indexOf('sketch');
///     if (sketchIndex < 0) return false;
///     const sketchedMove = {
///         move: move.name,
///         id: move.id,
///         pp: move.pp,
///         maxpp: move.pp,
///         target: move.target,
///         disabled: false,
///         used: false,
///     };
///     source.moveSlots[sketchIndex] = sketchedMove;
///     source.baseMoveSlots[sketchIndex] = sketchedMove;
///     this.add('-activate', source, 'move: Sketch', move.name);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

