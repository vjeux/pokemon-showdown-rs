//! Spite Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(target) {            let move: Move | ActiveMove | null = target.lastMove;
///             if (!move || move.isZ) return false;
///             if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
/// 
///             const ppDeducted = target.deductPP(move.id, 4);
///             if (!ppDeducted) return false;
///             this.add("-activate", target, 'move: Spite', move.name, ppDeducted);
///         }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

