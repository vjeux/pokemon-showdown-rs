//! Belch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	belch: {
//! 		num: 562,
//! 		accuracy: 90,
//! 		basePower: 120,
//! 		category: "Special",
//! 		name: "Belch",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, failmefirst: 1, nosleeptalk: 1, noassist: 1, failcopycat: 1, failmimic: 1, failinstruct: 1 },
//! 		onDisableMove(pokemon) {
//! 			if (!pokemon.ateBerry) pokemon.disableMove('belch');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Poison",
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onDisableMove(...)
pub fn on_disable_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

