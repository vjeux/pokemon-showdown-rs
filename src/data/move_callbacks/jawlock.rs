//! Jaw Lock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	jawlock: {
//! 		num: 746,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Physical",
//! 		name: "Jaw Lock",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1, bite: 1 },
//! 		onHit(target, source, move) {
//! 			source.addVolatile('trapped', target, move, 'trapper');
//! 			target.addVolatile('trapped', source, move, 'trapper');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Dark",
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

