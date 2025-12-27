//! Light Metal Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	lightmetal: {
//! 		onModifyWeight(weighthg) {
//! 			return this.trunc(weighthg / 2);
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Light Metal",
//! 		rating: 1,
//! 		num: 135,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyWeight(weighthg)
/// Halves the Pokemon's weight
pub fn on_modify_weight(weighthg: i32) -> AbilityHandlerResult {
    // return this.trunc(weighthg / 2);
    AbilityHandlerResult::Number((weighthg / 2) as i32)
}

