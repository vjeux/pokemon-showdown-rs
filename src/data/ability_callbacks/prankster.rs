//! Prankster Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	prankster: {
//! 		onModifyPriority(priority, pokemon, target, move) {
//! 			if (move?.category === 'Status') {
//! 				move.pranksterBoosted = true;
//! 				return priority + 1;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Prankster",
//! 		rating: 4,
//! 		num: 158,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyPriority(priority, pokemon, target, move)
pub fn on_modify_priority(_battle: &Battle, priority: i32, _pokemon: &Pokemon, _target: Option<&Pokemon>, move_: &mut MoveDef) -> AbilityHandlerResult {
    // if (move?.category === 'Status')
    if move_.category == MoveCategory::Status {
        // move.pranksterBoosted = true;
        move_.prankster_boosted = true;
        // return priority + 1;
        return AbilityHandlerResult::Number(priority + 1);
    }
    AbilityHandlerResult::Undefined
}
