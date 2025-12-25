//! Strong Jaw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	strongjaw: {
//! 		onBasePowerPriority: 19,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.flags['bite']) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Strong Jaw",
//! 		rating: 3.5,
//! 		num: 173,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 19;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts biting moves by 1.5x
pub fn on_base_power(_base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['bite'])
    if move_.flags.bite {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

