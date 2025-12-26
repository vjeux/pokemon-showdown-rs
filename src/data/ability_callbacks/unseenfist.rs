//! Unseen Fist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	unseenfist: {
//! 		onModifyMove(move) {
//! 			if (move.flags['contact']) delete move.flags['protect'];
//! 		},
//! 		flags: {},
//! 		name: "Unseen Fist",
//! 		rating: 2,
//! 		num: 260,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyMove(move)
/// Contact moves ignore protection (Protect, Detect, etc.)
pub fn on_modify_move(_battle: &mut Battle, move_: &mut MoveDef) -> AbilityHandlerResult {
    // if (move.flags['contact']) delete move.flags['protect'];
    if move_.flags.contact {
        move_.flags.protect = false;
    }
    AbilityHandlerResult::Undefined
}

