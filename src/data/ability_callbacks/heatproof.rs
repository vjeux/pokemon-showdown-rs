//! Heatproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	heatproof: {
//! 		onSourceModifyAtkPriority: 6,
//! 		onSourceModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Fire') {
//! 				this.debug('Heatproof Atk weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onSourceModifySpAPriority: 5,
//! 		onSourceModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Fire') {
//! 				this.debug('Heatproof SpA weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onDamage(damage, target, source, effect) {
//! 			if (effect && effect.id === 'brn') {
//! 				return damage / 2;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Heatproof",
//! 		rating: 2,
//! 		num: 85,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyAtkPriority: 6
pub const ON_SOURCE_MODIFY_ATK_PRIORITY: i32 = 6;
/// onSourceModifySpAPriority: 5
pub const ON_SOURCE_MODIFY_SPA_PRIORITY: i32 = 5;

/// onSourceModifyAtk(atk, attacker, defender, move)
/// Halves Fire-type attack damage
pub fn on_source_modify_atk(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Fire')
    if move_.move_type == "Fire" {
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

/// onSourceModifySpA(atk, attacker, defender, move)
/// Halves Fire-type special attack damage
pub fn on_source_modify_sp_a(_spa: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Fire')
    if move_.move_type == "Fire" {
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

/// onDamage(damage, target, source, effect)
/// Halves burn damage
pub fn on_damage(damage: u32, _target: &Pokemon, _source: &Pokemon, effect: &Effect) -> AbilityHandlerResult {
    // if (effect && effect.id === 'brn')
    if effect.id == "brn" {
        // return damage / 2;
        return AbilityHandlerResult::Number((damage / 2) as i32);
    }
    AbilityHandlerResult::Undefined
}

