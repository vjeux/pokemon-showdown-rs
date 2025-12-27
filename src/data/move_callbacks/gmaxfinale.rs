//! G-Max Finale Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {
/// 				for (const pokemon of source.alliesAndSelf()) {
/// 					this.heal(pokemon.maxhp / 6, pokemon, source, move);
/// 				}
/// 			},
/// 
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

