//! Gluttony Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	gluttony: {
//! 		onStart(pokemon) {
//! 			pokemon.abilityState.gluttony = true;
//! 		},
//! 		onDamage(item, pokemon) {
//! 			pokemon.abilityState.gluttony = true;
//! 		},
//! 		flags: {},
//! 		name: "Gluttony",
//! 		rating: 1.5,
//! 		num: 82,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onDamage(...)
pub fn on_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

