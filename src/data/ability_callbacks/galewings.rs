//! Gale Wings Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	galewings: {
//! 		onModifyPriority(priority, pokemon, target, move) {
//! 			if (move?.type === 'Flying' && pokemon.hp === pokemon.maxhp) return priority + 1;
//! 		},
//! 		flags: {},
//! 		name: "Gale Wings",
//! 		rating: 1.5,
//! 		num: 177,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyPriority(priority, pokemon, target, move)
pub fn on_modify_priority(_battle: &Battle, priority: i32, pokemon: &Pokemon, _target: Option<&Pokemon>, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move?.type === 'Flying' && pokemon.hp === pokemon.maxhp) return priority + 1;
    if move_.move_type == "Flying" && pokemon.hp == pokemon.maxhp {
        return AbilityHandlerResult::Number(priority + 1);
    }
    AbilityHandlerResult::Undefined
}
