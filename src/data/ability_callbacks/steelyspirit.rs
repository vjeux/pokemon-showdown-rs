//! Steely Spirit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	steelyspirit: {
//! 		onAllyBasePowerPriority: 22,
//! 		onAllyBasePower(basePower, attacker, defender, move) {
//! 			if (move.type === 'Steel') {
//! 				this.debug('Steely Spirit boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Steely Spirit",
//! 		rating: 3.5,
//! 		num: 252,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_ALLY_BASE_POWER_PRIORITY: i32 = 22;

/// onAllyBasePower(basePower, attacker, defender, move)
/// Boosts Steel-type moves used by allies by 1.5x
pub fn on_ally_base_power(_battle: &mut Battle, _base_power: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // Ensure this is an ally, not self
    let attacker_ref = (attacker.side_index, attacker.position);
    let holder_ref = (ability_holder.side_index, ability_holder.position);

    if attacker_ref != holder_ref {
        // if (move.type === 'Steel')
        if move_.move_type == "Steel" {
            // this.debug('Steely Spirit boost');
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
    }

    AbilityHandlerResult::Undefined
}

