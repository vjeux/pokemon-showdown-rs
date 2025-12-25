//! Long Reach Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	longreach: {
//! 		onModifyMove(move) {
//! 			delete move.flags['contact'];
//! 		},
//! 		flags: {},
//! 		name: "Long Reach",
//! 		rating: 1,
//! 		num: 203,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyMove(move)
/// Removes contact flag from all moves
///
/// TODO: onModifyMove handler not yet implemented
/// When implemented, should:
/// 1. Delete move.flags['contact'] to make moves non-contact
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

