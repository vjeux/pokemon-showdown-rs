//! Mega Launcher Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	megalauncher: {
//! 		onBasePowerPriority: 19,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.flags['pulse']) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Mega Launcher",
//! 		rating: 3,
//! 		num: 178,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 19;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts pulse moves by 1.5x
pub fn on_base_power(_base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['pulse'])
    if move_.flags.pulse {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

