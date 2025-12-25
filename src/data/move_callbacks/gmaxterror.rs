//! G-Max Terror Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	gmaxterror: {
//! 		num: 1000,
//! 		accuracy: true,
//! 		basePower: 10,
//! 		category: "Physical",
//! 		isNonstandard: "Gigantamax",
//! 		name: "G-Max Terror",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: {},
//! 		isMax: "Gengar",
//! 		self: {
//! 			onHit(source) {
//! 				for (const pokemon of source.foes()) {
//! 					pokemon.addVolatile('trapped', source, null, 'trapper');
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "adjacentFoe",
//! 		type: "Ghost",
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

