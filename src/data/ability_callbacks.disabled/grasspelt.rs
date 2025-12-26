//! Grass Pelt Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	grasspelt: {
//! 		onModifyDefPriority: 6,
//! 		onModifyDef(pokemon) {
//! 			if (this.field.isTerrain("grassyterrain")) return this.chainModify(1.5);
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Grass Pelt",
//! 		rating: 0.5,
//! 		num: 179,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_DEF_PRIORITY: i32 = 6;

/// onModifyDef(pokemon)
/// Boosts Defense by 1.5x in Grassy Terrain
pub fn on_modify_def(battle: &mut Battle, _def: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.field.isTerrain("grassyterrain"))
    if battle.field.is_terrain("grassyterrain") {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}
