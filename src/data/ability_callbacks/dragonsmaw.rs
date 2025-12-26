//! Dragon's Maw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	dragonsmaw: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Dragon') {
//! 				this.debug('Dragon\'s Maw boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Dragon') {
//! 				this.debug('Dragon\'s Maw boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Dragon's Maw",
//! 		rating: 3.5,
//! 		num: 263,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyAtkPriority: 5
pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;
/// onModifySpAPriority: 5
pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender, move)
/// Boosts Dragon-type moves by 1.5x
pub fn on_modify_atk(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Dragon')
    if move_.move_type == "Dragon" {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

/// onModifySpA(atk, attacker, defender, move)
/// Boosts Dragon-type moves by 1.5x
pub fn on_modify_sp_a(_spa: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Dragon')
    if move_.move_type == "Dragon" {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

