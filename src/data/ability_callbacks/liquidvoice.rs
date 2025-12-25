//! Liquid Voice Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	liquidvoice: {
//! 		onModifyTypePriority: -1,
//! 		onModifyType(move, pokemon) {
//! 			if (move.flags['sound'] && !pokemon.volatiles['dynamax']) { // hardcode
//! 				move.type = 'Water';
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Liquid Voice",
//! 		rating: 1.5,
//! 		num: 204,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_TYPE_PRIORITY: i32 = -1;

/// onModifyType(move, pokemon)
/// Changes sound moves to Water-type
pub fn on_modify_type(move_: &mut MoveDef, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (move.flags['sound'] && !pokemon.volatiles['dynamax'])
    if move_.flags.sound && !pokemon.volatiles.contains_key(&ID::new("dynamax")) {
        // move.type = 'Water';
        move_.move_type = "Water".to_string();
    }
    AbilityHandlerResult::Undefined
}

