//! Seed Sower Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	seedsower: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			this.field.setTerrain('grassyterrain');
//! 		},
//! 		flags: {},
//! 		name: "Seed Sower",
//! 		rating: 2.5,
//! 		num: 269,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(...)
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

