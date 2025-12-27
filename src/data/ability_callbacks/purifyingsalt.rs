//! Purifying Salt Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	purifyingsalt: {
//! 		onSetStatus(status, target, source, effect) {
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Purifying Salt');
//! 			}
//! 			return false;
//! 		},
//! 		onTryAddVolatile(status, target) {
//! 			if (status.id === 'yawn') {
//! 				this.add('-immune', target, '[from] ability: Purifying Salt');
//! 				return null;
//! 			}
//! 		},
//! 		onSourceModifyAtkPriority: 6,
//! 		onSourceModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Ghost') {
//! 				this.debug('Purifying Salt weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onSourceModifySpAPriority: 5,
//! 		onSourceModifySpA(spa, attacker, defender, move) {
//! 			if (move.type === 'Ghost') {
//! 				this.debug('Purifying Salt weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Purifying Salt",
//! 		rating: 4,
//! 		num: 272,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSetStatus(status, target, source, effect)
/// Prevents all status conditions
pub fn on_set_status(battle: &mut Battle, _status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if ((effect as Move)?.status)
    if effect.status.is_some() {
        // this.add('-immune', target, '[from] ability: Purifying Salt');
        battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Purifying Salt")]);
    }
    // return false;
    AbilityHandlerResult::False
}

/// onTryAddVolatile(status, target)
/// Prevents Yawn
pub fn on_try_add_volatile(battle: &mut Battle, status: &Status, target: &Pokemon) -> AbilityHandlerResult {
    // if (status.id === 'yawn')
    if status.id == "yawn" {
        // this.add('-immune', target, '[from] ability: Purifying Salt');
        battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Purifying Salt")]);
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

pub const ON_SOURCE_MODIFY_ATK_PRIORITY: i32 = 6;

/// onSourceModifyAtk(atk, attacker, defender, move)
/// Halves damage from Ghost-type moves (Atk)
pub fn on_source_modify_atk(_atk: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Ghost')
    if move_.move_type == "Ghost" {
        // this.debug('Purifying Salt weaken');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

pub const ON_SOURCE_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onSourceModifySpA(spa, attacker, defender, move)
/// Halves damage from Ghost-type moves (SpA)
pub fn on_source_modify_sp_a(_spa: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Ghost')
    if move_.move_type == "Ghost" {
        // this.debug('Purifying Salt weaken');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

