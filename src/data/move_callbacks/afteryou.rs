//! After You Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	afteryou: {
//! 		num: 495,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "After You",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { bypasssub: 1, allyanim: 1 },
//! 		onHit(target) {
//! 			if (this.activePerHalf === 1) return false; // fails in singles
//! 			const action = this.queue.willMove(target);
//! 			if (action) {
//! 				this.queue.prioritizeAction(action);
//! 				this.add('-activate', target, 'move: After You');
//! 			} else {
//! 				return false;
//! 			}
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { boost: { spe: 1 } },
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

