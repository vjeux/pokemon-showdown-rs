//! Swarm Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	swarm: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3) {
//! 				this.debug('Swarm boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3) {
//! 				this.debug('Swarm boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Swarm",
//! 		rating: 2,
//! 		num: 68,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender, move)
/// Boosts Bug-type moves by 1.5x when HP <= 1/3
pub fn on_modify_atk(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3)
    if move_.move_type == "Bug" && attacker.hp <= attacker.maxhp / 3 {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(atk, attacker, defender, move)
/// Boosts Bug-type moves by 1.5x when HP <= 1/3
pub fn on_modify_sp_a(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Bug' && attacker.hp <= attacker.maxhp / 3)
    if move_.move_type == "Bug" && attacker.hp <= attacker.maxhp / 3 {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

