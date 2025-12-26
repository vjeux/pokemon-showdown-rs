//! Rivalry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	rivalry: {
//! 		onBasePowerPriority: 24,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (attacker.gender && defender.gender) {
//! 				if (attacker.gender === defender.gender) {
//! 					this.debug('Rivalry boost');
//! 					return this.chainModify(1.25);
//! 				} else {
//! 					this.debug('Rivalry weaken');
//! 					return this.chainModify(0.75);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rivalry",
//! 		rating: 0,
//! 		num: 79,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::{ID, Gender};
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 24;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts damage by 1.25x against same gender, weakens by 0.75x against opposite gender
pub fn on_base_power(_base_power: u32, attacker: &Pokemon, defender: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // if (attacker.gender && defender.gender)
    if attacker.gender != Gender::None && defender.gender != Gender::None {
        // if (attacker.gender === defender.gender)
        if attacker.gender == defender.gender {
            // this.debug('Rivalry boost');
            // return this.chainModify(1.25);
            return AbilityHandlerResult::ChainModify(5120, 4096); // 1.25x
        } else {
            // this.debug('Rivalry weaken');
            // return this.chainModify(0.75);
            return AbilityHandlerResult::ChainModify(3072, 4096); // 0.75x
        }
    }
    AbilityHandlerResult::Undefined
}

