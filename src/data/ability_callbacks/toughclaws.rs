//! Tough Claws Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	toughclaws: {
//! 		onBasePowerPriority: 21,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.flags['contact']) {
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Tough Claws",
//! 		rating: 3.5,
//! 		num: 181,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 21;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts contact moves by 1.3x
pub fn on_base_power(_base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['contact'])
    if move_.flags.contact {
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

