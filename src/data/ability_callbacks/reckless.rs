//! Reckless Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	reckless: {
//! 		onBasePowerPriority: 23,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.recoil || move.hasCrashDamage) {
//! 				this.debug('Reckless boost');
//! 				return this.chainModify([4915, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Reckless",
//! 		rating: 3,
//! 		num: 120,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts recoil moves by 1.2x
pub fn on_base_power(_base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.recoil || move.hasCrashDamage)
    if move_.recoil.is_some() || move_.has_crash_damage {
        // return this.chainModify([4915, 4096]);
        return AbilityHandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    AbilityHandlerResult::Undefined
}

