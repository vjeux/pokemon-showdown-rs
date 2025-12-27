//! Sketch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

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
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

