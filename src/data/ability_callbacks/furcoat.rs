//! Fur Coat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	furcoat: {
//! 		onModifyDefPriority: 6,
//! 		onModifyDef(def) {
//! 			return this.chainModify(2);
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Fur Coat",
//! 		rating: 4,
//! 		num: 169,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyDefPriority: 6
pub const ON_MODIFY_DEF_PRIORITY: i32 = 6;

/// onModifyDef(def)
/// Doubles Defense
pub fn on_modify_def(_def: i32) -> AbilityHandlerResult {
    // return this.chainModify(2);
    AbilityHandlerResult::ChainModify(8192, 4096) // 2x
}

