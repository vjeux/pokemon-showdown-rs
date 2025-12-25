//! G-Max Centiferno Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	gmaxcentiferno: {
//! 		num: 1000,
//! 		accuracy: true,
//! 		basePower: 10,
//! 		category: "Physical",
//! 		isNonstandard: "Gigantamax",
//! 		name: "G-Max Centiferno",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: {},
//! 		isMax: "Centiskorch",
//! 		self: {
//! 			onHit(source) {
//! 				for (const pokemon of source.foes()) {
//! 					pokemon.addVolatile('partiallytrapped', source, this.dex.getActiveMove('G-Max Centiferno'));
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "adjacentFoe",
//! 		type: "Fire",
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

