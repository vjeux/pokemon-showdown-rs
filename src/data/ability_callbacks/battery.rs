//! Battery Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	battery: {
//! 		onAllyBasePowerPriority: 22,
//! 		onAllyBasePower(basePower, attacker, defender, move) {
//! 			if (attacker !== this.effectState.target && move.category === 'Special') {
//! 				this.debug('Battery boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Battery",
//! 		rating: 0,
//! 		num: 217,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_ALLY_BASE_POWER_PRIORITY: i32 = 22;

/// onAllyBasePower(basePower, attacker, defender, move)
/// Boosts allies' Special moves by 1.3x
pub fn on_ally_base_power(_base_power: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // if (attacker !== this.effectState.target && move.category === 'Special')
    if attacker.position != ability_holder.position && move_.category == MoveCategory::Special {
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

