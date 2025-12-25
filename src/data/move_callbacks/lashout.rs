//! Lash Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	lashout: {
//! 		num: 808,
//! 		accuracy: 100,
//! 		basePower: 75,
//! 		category: "Physical",
//! 		name: "Lash Out",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1 },
//! 		onBasePower(basePower, source) {
//! 			if (source.statsLoweredThisTurn) {
//! 				this.debug('lashout buff');
//! 				return this.chainModify(2);
//! 			}
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

/// onBasePower(...)
pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

