//! Transform Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	transform: {
//! 		num: 144,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Transform",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { allyanim: 1, failencore: 1, noassist: 1, failcopycat: 1, failmimic: 1, failinstruct: 1 },
//! 		onHit(target, pokemon) {
//! 			if (!pokemon.transformInto(target)) {
//! 				return false;
//! 			}
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { effect: 'heal' },
//! 		contestType: "Clever",
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

