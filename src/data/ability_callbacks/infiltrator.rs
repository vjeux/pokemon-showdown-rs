//! Infiltrator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	infiltrator: {
//! 		onModifyMove(move) {
//! 			move.infiltrates = true;
//! 		},
//! 		flags: {},
//! 		name: "Infiltrator",
//! 		rating: 2.5,
//! 		num: 151,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyMove(move)
/// Makes moves bypass screens and substitutes
pub fn on_modify_move(_battle: &mut Battle, move_: &mut MoveDef) -> AbilityHandlerResult {
    // move.infiltrates = true;
    move_.infiltrates = true;
    AbilityHandlerResult::Undefined
}

