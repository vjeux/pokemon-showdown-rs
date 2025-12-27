//! Flying Press Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onEffectiveness(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onEffectiveness(typeMod, target, type, move) {			return typeMod + this.dex.getEffectiveness('Flying', type);
/// 		}
/// ```
pub fn on_effectiveness(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

