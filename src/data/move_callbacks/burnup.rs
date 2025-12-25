//! Burn Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	burnup: {
//! 		num: 682,
//! 		accuracy: 100,
//! 		basePower: 130,
//! 		category: "Special",
//! 		isNonstandard: "Unobtainable",
//! 		name: "Burn Up",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1, defrost: 1, metronome: 1 },
//! 		onTryMove(pokemon, target, move) {
//! 			if (pokemon.hasType('Fire')) return;
//! 			this.add('-fail', pokemon, 'move: Burn Up');
//! 			this.attrLastMove('[still]');
//! 			return null;
//! 		},
//! 		self: {
//! 			onHit(pokemon) {
//! 				pokemon.setType(pokemon.getTypes(true).map(type => type === "Fire" ? "???" : type));
//! 				this.add('-start', pokemon, 'typechange', pokemon.getTypes().join('/'), '[from] move: Burn Up');
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Fire",
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

