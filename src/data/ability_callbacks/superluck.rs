//! Super Luck Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	superluck: {
//! 		onModifyCritRatio(critRatio) {
//! 			return critRatio + 1;
//! 		},
//! 		flags: {},
//! 		name: "Super Luck",
//! 		rating: 1.5,
//! 		num: 105,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyCritRatio(critRatio)
pub fn on_modify_crit_ratio(crit_ratio: i32) -> AbilityHandlerResult {
    // return critRatio + 1;
    AbilityHandlerResult::Number(crit_ratio + 1)
}
