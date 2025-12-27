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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Sets Grassy Terrain when hit by a damaging move
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, _target: &Pokemon, _source: &mut Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // this.field.setTerrain('grassyterrain');
    battle.field.set_terrain(ID::new("grassyterrain"), None);
    AbilityHandlerResult::Undefined
}

