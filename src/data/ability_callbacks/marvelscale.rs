//! Marvel Scale Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	marvelscale: {
//! 		onModifyDefPriority: 6,
//! 		onModifyDef(def, pokemon) {
//! 			if (pokemon.status) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Marvel Scale",
//! 		rating: 2.5,
//! 		num: 63,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_DEF_PRIORITY: i32 = 6;

/// onModifyDef(def, pokemon)
/// Boosts Defense by 1.5x when statused
pub fn on_modify_def(_def: u32, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status)
    if !pokemon.status.is_empty() {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

