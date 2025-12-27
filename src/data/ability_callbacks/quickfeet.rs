//! Quick Feet Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	quickfeet: {
//! 		onModifySpe(spe, pokemon) {
//! 			if (pokemon.status) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Quick Feet",
//! 		rating: 2.5,
//! 		num: 95,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe, pokemon)
pub fn on_modify_spe(_battle: &Battle, _spe: i32, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status)
    if !pokemon.status.is_empty() {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}
