//! Sand Force Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sandforce: {
//! 		onBasePowerPriority: 21,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (this.field.isWeather('sandstorm')) {
//! 				if (move.type === 'Rock' || move.type === 'Ground' || move.type === 'Steel') {
//! 					this.debug('Sand Force boost');
//! 					return this.chainModify([5325, 4096]);
//! 				}
//! 			}
//! 		},
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'sandstorm') return false;
//! 		},
//! 		flags: {},
//! 		name: "Sand Force",
//! 		rating: 2,
//! 		num: 159,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 21;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts Rock/Ground/Steel moves by 1.3x in sandstorm
pub fn on_base_power(battle: &mut Battle, _base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.field.isWeather('sandstorm'))
    if battle.field.is_weather("sandstorm") {
        // if (move.type === 'Rock' || move.type === 'Ground' || move.type === 'Steel')
        if move_.move_type == "Rock" || move_.move_type == "Ground" || move_.move_type == "Steel" {
            // this.debug('Sand Force boost');
            // return this.chainModify([5325, 4096]);
            return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
        }
    }
    AbilityHandlerResult::Undefined
}

/// onImmunity(type, pokemon)
/// Immune to sandstorm damage
pub fn on_immunity(_battle: &mut Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'sandstorm') return false;
    if immunity_type == "sandstorm" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

