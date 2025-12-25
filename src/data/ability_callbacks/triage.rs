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

/// onModifyPriority(...)
pub fn on_modify_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

