//! Max Overgrowth Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	maxovergrowth: {
//! 		num: 773,
//! 		accuracy: true,
//! 		basePower: 10,
//! 		category: "Physical",
//! 		isNonstandard: "Past",
//! 		name: "Max Overgrowth",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: {},
//! 		isMax: true,
//! 		self: {
//! 			onHit(source) {
//! 				if (!source.volatiles['dynamax']) return;
//! 				this.field.setTerrain('grassyterrain');
//! 			},
//! 		},
//! 		target: "adjacentFoe",
//! 		type: "Grass",
//! 		contestType: "Cool",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

