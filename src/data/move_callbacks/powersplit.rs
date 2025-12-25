//! Power Split Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	powersplit: {
//! 		num: 471,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Power Split",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, allyanim: 1, metronome: 1 },
//! 		onHit(target, source) {
//! 			const newatk = Math.floor((target.storedStats.atk + source.storedStats.atk) / 2);
//! 			target.storedStats.atk = newatk;
//! 			source.storedStats.atk = newatk;
//! 			const newspa = Math.floor((target.storedStats.spa + source.storedStats.spa) / 2);
//! 			target.storedStats.spa = newspa;
//! 			source.storedStats.spa = newspa;
//! 			this.add('-activate', source, 'move: Power Split', `[of] ${target}`);
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Psychic",
//! 		zMove: { boost: { spe: 1 } },
//! 		contestType: "Clever",
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

