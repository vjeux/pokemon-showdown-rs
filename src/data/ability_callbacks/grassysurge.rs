//! Grassy Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	grassysurge: {
//! 		onStart(source) {
//! 			this.field.setTerrain('grassyterrain');
//! 		},
//! 		flags: {},
//! 		name: "Grassy Surge",
//! 		rating: 4,
//! 		num: 229,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setTerrain('grassyterrain');
    battle.field.set_terrain(ID::new("grassyterrain"), None);
    AbilityHandlerResult::Undefined
}
