//! Triage Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	triage: {
//! 		onModifyPriority(priority, pokemon, target, move) {
//! 			if (move?.flags['heal']) return priority + 3;
//! 		},
//! 		flags: {},
//! 		name: "Triage",
//! 		rating: 3.5,
//! 		num: 205,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyPriority(priority, pokemon, target, move)
pub fn on_modify_priority(_battle: &Battle, priority: i32, _pokemon: &Pokemon, _target: Option<&Pokemon>, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move?.flags['heal']) return priority + 3;
    if move_.flags.heal {
        return AbilityHandlerResult::Number(priority + 3);
    }
    AbilityHandlerResult::Undefined
}
