//! Heavy Metal Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	heavymetal: {
//! 		onModifyWeightPriority: 1,
//! 		onModifyWeight(weighthg) {
//! 			return weighthg * 2;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Heavy Metal",
//! 		rating: 0,
//! 		num: 134,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyWeightPriority: 1
pub const ON_MODIFY_WEIGHT_PRIORITY: i32 = 1;

/// onModifyWeight(weighthg)
/// Doubles the Pokemon's weight
pub fn on_modify_weight(weighthg: u32) -> AbilityHandlerResult {
    // return weighthg * 2;
    AbilityHandlerResult::Number((weighthg * 2) as i32)
}

