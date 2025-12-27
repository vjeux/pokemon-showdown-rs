//! Burn Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryMove(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onTryMove(pokemon, target, move) {			if (pokemon.hasType('Fire')) return;
/// 			this.add('-fail', pokemon, 'move: Burn Up');
/// 			this.attrLastMove('[still]');
/// 			return null;
/// 		}
/// ```
pub fn on_try_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

