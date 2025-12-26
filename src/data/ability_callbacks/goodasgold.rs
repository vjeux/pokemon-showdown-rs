//! Good as Gold Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	goodasgold: {
//! 		onTryHit(target, source, move) {
//! 			if (move.category === 'Status' && target !== source) {
//! 				this.add('-immune', target, '[from] ability: Good as Gold');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Good as Gold",
//! 		rating: 5,
//! 		num: 283,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Immune to Status moves from other Pokemon
pub fn on_try_hit(target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Status' && target !== source)
    if move_.category == MoveCategory::Status && target.position != source.position {
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

