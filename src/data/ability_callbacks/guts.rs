//! Guts Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	guts: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, pokemon) {
//! 			if (pokemon.status) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Guts",
//! 		rating: 3.5,
//! 		num: 62,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyAtkPriority: 5
pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, pokemon)
/// Boosts Attack by 1.5x when statused
pub fn on_modify_atk(_atk: u32, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status)
    if !pokemon.status.is_empty() {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

