//! Nature Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	naturepower: {
//! 		num: 267,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Nature Power",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { failencore: 1, nosleeptalk: 1, noassist: 1, failcopycat: 1, failmimic: 1, failinstruct: 1 },
//! 		onTryHit(target, pokemon) {
//! 			let move = 'triattack';
//! 			if (this.field.isTerrain('electricterrain')) {
//! 				move = 'thunderbolt';
//! 			} else if (this.field.isTerrain('grassyterrain')) {
//! 				move = 'energyball';
//! 			} else if (this.field.isTerrain('mistyterrain')) {
//! 				move = 'moonblast';
//! 			} else if (this.field.isTerrain('psychicterrain')) {
//! 				move = 'psychic';
//! 			}
//! 			this.actions.useMove(move, pokemon, { target });
//! 			return null;
//! 		},
//! 		callsMove: true,
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		contestType: "Beautiful",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit(...)
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

