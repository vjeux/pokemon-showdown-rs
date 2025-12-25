//! G-Max Depletion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	gmaxdepletion: {
//! 		num: 1000,
//! 		accuracy: true,
//! 		basePower: 10,
//! 		category: "Physical",
//! 		isNonstandard: "Gigantamax",
//! 		name: "G-Max Depletion",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: {},
//! 		isMax: "Duraludon",
//! 		self: {
//! 			onHit(source) {
//! 				for (const pokemon of source.foes()) {
//! 					let move: Move | ActiveMove | null = pokemon.lastMove;
//! 					if (!move || move.isZ) continue;
//! 					if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
//! 
//! 					const ppDeducted = pokemon.deductPP(move.id, 2);
//! 					if (ppDeducted) {
//! 						this.add("-activate", pokemon, 'move: G-Max Depletion', move.name, ppDeducted);
//! 						// Don't return here because returning early doesn't trigger
//! 						// activation text for the second Pokemon in doubles
//! 					}
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "adjacentFoe",
//! 		type: "Dragon",
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

