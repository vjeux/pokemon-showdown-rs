//! Psychic Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	psychicsurge: {
//! 		onStart(source) {
//! 			this.field.setTerrain('psychicterrain');
//! 		},
//! 		flags: {},
//! 		name: "Psychic Surge",
//! 		rating: 4,
//! 		num: 227,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setTerrain('psychicterrain');
    battle.field.set_terrain(ID::new("psychicterrain"), None);
    AbilityHandlerResult::Undefined
}
