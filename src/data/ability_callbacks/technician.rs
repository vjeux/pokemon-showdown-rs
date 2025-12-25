//! Technician Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	technician: {
//! 		onBasePowerPriority: 30,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
//! 			this.debug(`Base Power: ${basePowerAfterMultiplier}`);
//! 			if (basePowerAfterMultiplier <= 60) {
//! 				this.debug('Technician boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Technician",
//! 		rating: 3.5,
//! 		num: 101,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onBasePowerPriority: 30
pub const ON_BASE_POWER_PRIORITY: i32 = 30;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts base power of moves with 60 or less base power by 1.5x
///
/// Note: JS checks basePowerAfterMultiplier using this.modify(basePower, this.event.modifier)
/// For now, implementing simple check against base_power parameter
/// TODO: May need to account for prior modifiers in the damage calculation chain
pub fn on_base_power(_battle: &Battle, base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // const basePowerAfterMultiplier = this.modify(basePower, this.event.modifier);
    // if (basePowerAfterMultiplier <= 60)
    // Note: Simplified - checking base_power directly. May need event.modifier support later.
    if base_power <= 60 {
        // this.debug('Technician boost');
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

