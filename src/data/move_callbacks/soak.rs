//! Soak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	soak: {
//! 		num: 487,
//! 		accuracy: 100,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Soak",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { protect: 1, reflectable: 1, mirror: 1, allyanim: 1, metronome: 1 },
//! 		onHit(target) {
//! 			if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
//! 				// Soak should animate even when it fails.
//! 				// Returning false would suppress the animation.
//! 				this.add('-fail', target);
//! 				return null;
//! 			}
//! 			this.add('-start', target, 'typechange', 'Water');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Water",
//! 		zMove: { boost: { spa: 1 } },
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

