//! Iron Fist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	ironfist: {
//! 		onBasePowerPriority: 23,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.flags['punch']) {
//! 				this.debug('Iron Fist boost');
//! 				return this.chainModify([4915, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Iron Fist",
//! 		rating: 3,
//! 		num: 89,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts punching moves by 1.2x
pub fn on_base_power(_base_power: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['punch'])
    if move_.flags.punch {
        // return this.chainModify([4915, 4096]);
        return AbilityHandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    AbilityHandlerResult::Undefined
}

