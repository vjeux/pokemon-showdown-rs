//! G-Max Finale Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	gmaxfinale: {
//! 		num: 1000,
//! 		accuracy: true,
//! 		basePower: 10,
//! 		category: "Physical",
//! 		isNonstandard: "Gigantamax",
//! 		name: "G-Max Finale",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: {},
//! 		isMax: "Alcremie",
//! 		self: {
//! 			onHit(target, source, move) {
//! 				for (const pokemon of source.alliesAndSelf()) {
//! 					this.heal(pokemon.maxhp / 6, pokemon, source, move);
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "adjacentFoe",
//! 		type: "Fairy",
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

