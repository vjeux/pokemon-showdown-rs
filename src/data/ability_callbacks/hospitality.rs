//! Hospitality Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hospitality: {
//! 		onSwitchInPriority: -2,
//! 		onStart(pokemon) {
//! 			for (const ally of pokemon.adjacentAllies()) {
//! 				this.heal(ally.baseMaxhp / 4, ally, pokemon);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Hospitality",
//! 		rating: 0,
//! 		num: 299,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority(...)
pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

