//! Block Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	block: {
//! 		num: 335,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Block",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { reflectable: 1, mirror: 1, metronome: 1 },
//! 		onHit(target, source, move) {
//! 			return target.addVolatile('trapped', source, move, 'trapper');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { boost: { def: 1 } },
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

