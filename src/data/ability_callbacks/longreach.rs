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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyMove(move)
/// Removes contact flag from all moves
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // delete move.flags['contact'];
    move_.flags.contact = false;
    AbilityHandlerResult::Undefined
}

