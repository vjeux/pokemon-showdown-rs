//! Fake Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	fakeout: {
//! 		num: 252,
//! 		accuracy: 100,
//! 		basePower: 40,
//! 		category: "Physical",
//! 		name: "Fake Out",
//! 		pp: 10,
//! 		priority: 3,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1 },
//! 		onTry(source) {
//! 			if (source.activeMoveActions > 1) {
//! 				this.hint("Fake Out only works on your first turn out.");
//! 				return false;
//! 			}
//! 		},
//! 		secondary: {
//! 			chance: 100,
//! 			volatileStatus: 'flinch',
//! 		},
//! 		target: "normal",
//! 		type: "Normal",
//! 		contestType: "Cute",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(...)
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

