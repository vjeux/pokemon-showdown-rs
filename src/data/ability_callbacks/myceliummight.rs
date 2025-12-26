//! Mycelium Might Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	myceliummight: {
//! 		onFractionalPriorityPriority: -1,
//! 		onFractionalPriority(priority, pokemon, target, move) {
//! 			if (move.category === 'Status') {
//! 				return -0.1;
//! 			}
//! 		},
//! 		onModifyMove(move) {
//! 			if (move.category === 'Status') {
//! 				move.ignoreAbility = true;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Mycelium Might",
//! 		rating: 2,
//! 		num: 298,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_FRACTIONAL_PRIORITY_PRIORITY: i32 = -1;

/// onFractionalPriority(priority, pokemon, target, move)
/// Makes status moves slower (fractional priority -0.1)
pub fn on_fractional_priority(_battle: &mut Battle, _priority: f64, _pokemon: &Pokemon, _target: Option<&Pokemon>, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Status')
    if move_.category == MoveCategory::Status {
        // return -0.1;
        return AbilityHandlerResult::FractionalPriority(-0.1);
    }
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes status moves ignore abilities
///
/// TODO: onModifyMove handler not yet implemented
/// TODO: Needs move.ignoreAbility field
/// When implemented, should:
/// 1. Check if move.category === 'Status'
/// 2. Set move.ignoreAbility = true
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

