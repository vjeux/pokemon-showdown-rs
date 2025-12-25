//! Double Shock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	doubleshock: {
//! 		num: 892,
//! 		accuracy: 100,
//! 		basePower: 120,
//! 		category: "Physical",
//! 		name: "Double Shock",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1 },
//! 		onTryMove(pokemon, target, move) {
//! 			if (pokemon.hasType('Electric')) return;
//! 			this.add('-fail', pokemon, 'move: Double Shock');
//! 			this.attrLastMove('[still]');
//! 			return null;
//! 		},
//! 		self: {
//! 			onHit(pokemon) {
//! 				pokemon.setType(pokemon.getTypes(true).map(type => type === "Electric" ? "???" : type));
//! 				this.add('-start', pokemon, 'typechange', pokemon.getTypes().join('/'), '[from] move: Double Shock');
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Electric",
//! 		contestType: "Clever",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryMove(...)
pub fn on_try_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

