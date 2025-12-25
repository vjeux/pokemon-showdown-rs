//! Quick Draw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	quickdraw: {
//! 		onFractionalPriorityPriority: -1,
//! 		onFractionalPriority(priority, pokemon, target, move) {
//! 			if (move.category !== "Status" && this.randomChance(3, 10)) {
//! 				this.add('-activate', pokemon, 'ability: Quick Draw');
//! 				return 0.1;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Quick Draw",
//! 		rating: 2.5,
//! 		num: 259,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onFractionalPriorityPriority(...)
pub fn on_fractional_priority_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onFractionalPriority(...)
pub fn on_fractional_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

