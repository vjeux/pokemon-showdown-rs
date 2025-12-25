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

/// onStart(pokemon)
pub fn on_start(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // pokemon.abilityState.gluttony = true;
    pokemon.ability_state.data.insert("gluttony".to_string(), serde_json::Value::Bool(true));
    AbilityHandlerResult::Undefined
}

/// onDamage(item, pokemon)
pub fn on_damage(_damage: u32, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // pokemon.abilityState.gluttony = true;
    pokemon.ability_state.data.insert("gluttony".to_string(), serde_json::Value::Bool(true));
    AbilityHandlerResult::Undefined
}

