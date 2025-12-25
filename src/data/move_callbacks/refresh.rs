//! Refresh Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	refresh: {
//! 		num: 287,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Refresh",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { snatch: 1, metronome: 1 },
//! 		onHit(pokemon) {
//! 			if (['', 'slp', 'frz'].includes(pokemon.status)) return false;
//! 			pokemon.cureStatus();
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Normal",
//! 		zMove: { effect: 'heal' },
//! 		contestType: "Cute",
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

