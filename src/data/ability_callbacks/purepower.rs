//! Pure Power Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	purepower: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk) {
//! 			return this.chainModify(2);
//! 		},
//! 		flags: {},
//! 		name: "Pure Power",
//! 		rating: 5,
//! 		num: 74,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk)
/// Doubles the Pokemon's Attack stat
pub fn on_modify_atk(_atk: u32) -> AbilityHandlerResult {
    // return this.chainModify(2);
    AbilityHandlerResult::ChainModify(8192, 4096) // 2x
}

