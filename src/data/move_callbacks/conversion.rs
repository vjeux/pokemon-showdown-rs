//! Conversion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	conversion: {
//! 		num: 160,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Conversion",
//! 		pp: 30,
//! 		priority: 0,
//! 		flags: { snatch: 1, metronome: 1 },
//! 		onHit(target) {
//! 			const type = this.dex.moves.get(target.moveSlots[0].id).type;
//! 			if (target.hasType(type) || !target.setType(type)) return false;
//! 			this.add('-start', target, 'typechange', type);
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Normal",
//! 		zMove: { boost: { atk: 1, def: 1, spa: 1, spd: 1, spe: 1 } },
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

