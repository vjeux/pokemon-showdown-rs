//! Tri Attack Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	triattack: {
//! 		num: 161,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Special",
//! 		name: "Tri Attack",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1, metronome: 1 },
//! 		secondary: {
//! 			chance: 20,
//! 			onHit(target, source) {
//! 				const result = this.random(3);
//! 				if (result === 0) {
//! 					target.trySetStatus('brn', source);
//! 				} else if (result === 1) {
//! 					target.trySetStatus('par', source);
//! 				} else {
//! 					target.trySetStatus('frz', source);
//! 				}
//! 			},
//! 		},
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

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

