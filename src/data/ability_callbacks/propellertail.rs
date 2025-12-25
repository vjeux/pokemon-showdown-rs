//! Propeller Tail Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	propellertail: {
//! 		onModifyMovePriority: 1,
//! 		onModifyMove(move) {
//! 			// most of the implementation is in Battle#getTarget
//! 			move.tracksTarget = move.target !== 'scripted';
//! 		},
//! 		flags: {},
//! 		name: "Propeller Tail",
//! 		rating: 0,
//! 		num: 239,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_MOVE_PRIORITY: i32 = 1;

/// onModifyMove(move)
/// Ignores target redirection effects (most logic in Battle#getTarget)
pub fn on_modify_move(_battle: &mut Battle, move_: &mut MoveDef) -> AbilityHandlerResult {
    // most of the implementation is in Battle#getTarget
    // move.tracksTarget = move.target !== 'scripted';
    move_.tracks_target = move_.target != MoveTargetType::Scripted;
    AbilityHandlerResult::Undefined
}

