//! Flare Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	flareboost: {
//! 		onBasePowerPriority: 19,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (attacker.status === 'brn' && move.category === 'Special') {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Flare Boost",
//! 		rating: 2,
//! 		num: 138,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 19;

    /// onBasePower(basePower, attacker, defender, move)
    pub fn on_base_power(_base_power: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        if attacker.has_status("brn") && move_.category == MoveCategory::Special {
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }
