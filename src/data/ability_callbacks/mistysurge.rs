//! Misty Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mistysurge: {
//! 		onStart(source) {
//! 			this.field.setTerrain('mistyterrain');
//! 		},
//! 		flags: {},
//! 		name: "Misty Surge",
//! 		rating: 3.5,
//! 		num: 228,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setTerrain('mistyterrain');
    battle.field.set_terrain(ID::new("mistyterrain"), None);
    AbilityHandlerResult::Undefined
}
