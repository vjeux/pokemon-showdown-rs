//! Prankster Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	prankster: {
//! 		onModifyPriority(priority, pokemon, target, move) {
//! 			if (move?.category === 'Status') {
//! 				move.pranksterBoosted = true;
//! 				return priority + 1;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Prankster",
//! 		rating: 4,
//! 		num: 158,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyPriority(...)
pub fn on_modify_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

