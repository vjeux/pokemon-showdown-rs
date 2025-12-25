//! Round Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	round: {
//! 		num: 496,
//! 		accuracy: 100,
//! 		basePower: 60,
//! 		basePowerCallback(target, source, move) {
//! 			if (move.sourceEffect === 'round') {
//! 				this.debug('BP doubled');
//! 				return move.basePower * 2;
//! 			}
//! 			return move.basePower;
//! 		},
//! 		category: "Special",
//! 		name: "Round",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1, sound: 1, bypasssub: 1, metronome: 1 },
//! 		onTry(source, target, move) {
//! 			for (const action of this.queue.list as MoveAction[]) {
//! 				if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
//! 				if (action.move.id === 'round') {
//! 					this.queue.prioritizeAction(action, move);
//! 					return;
//! 				}
//! 			}
//! 		},
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

/// onTry(...)
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

