//! Power Spot Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	powerspot: {
//! 		onAllyBasePowerPriority: 22,
//! 		onAllyBasePower(basePower, attacker, defender, move) {
//! 			if (attacker !== this.effectState.target) {
//! 				this.debug('Power Spot boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Power Spot",
//! 		rating: 0,
//! 		num: 249,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_ALLY_BASE_POWER_PRIORITY: i32 = 22;

/// onAllyBasePower(basePower, attacker, defender, move)
/// Boosts ally move power by 1.3x
///
/// TODO: onAllyBasePower handler not yet called by battle engine
pub fn on_ally_base_power(_battle: &mut Battle, _base_power: i32, attacker: &Pokemon, _defender: &Pokemon, _move: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // if (attacker !== this.effectState.target)
    let attacker_ref = (attacker.side_index, attacker.position);
    let holder_ref = (ability_holder.side_index, ability_holder.position);

    if attacker_ref != holder_ref {
        // this.debug('Power Spot boost');
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // 1.3x boost
    }

    AbilityHandlerResult::Undefined
}

