//! Surge Surfer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	surgesurfer: {
//! 		onModifySpe(spe) {
//! 			if (this.field.isTerrain('electricterrain')) {
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Surge Surfer",
//! 		rating: 3,
//! 		num: 207,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe)
pub fn on_modify_spe(battle: &Battle, _spe: u32) -> AbilityHandlerResult {
    // if (this.field.isTerrain('electricterrain'))
    if *battle.field.get_terrain() == ID::new("electricterrain") {
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(2, 1);
    }
    AbilityHandlerResult::Undefined
}
