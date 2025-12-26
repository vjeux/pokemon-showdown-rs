//! Toxic Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	toxicboost: {
//! 		onBasePowerPriority: 19,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if ((attacker.status === 'psn' || attacker.status === 'tox') && move.category === 'Physical') {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Toxic Boost",
//! 		rating: 3,
//! 		num: 137,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onBasePowerPriority: 19
pub const ON_BASE_POWER_PRIORITY: i32 = 19;

/// onBasePower(basePower, attacker, defender, move)
pub fn on_base_power(_base_power: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if ((attacker.status === 'psn' || attacker.status === 'tox') && move.category === 'Physical')
    if (attacker.status.as_str() == "psn" || attacker.status.as_str() == "tox") && move_.category == MoveCategory::Physical {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x = 6144/4096
    }
    AbilityHandlerResult::Undefined
}
