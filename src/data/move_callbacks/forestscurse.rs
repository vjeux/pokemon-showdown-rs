//! Forest's Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	forestscurse: {
//! 		num: 571,
//! 		accuracy: 100,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Forest's Curse",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { protect: 1, reflectable: 1, mirror: 1, allyanim: 1, metronome: 1 },
//! 		onHit(target) {
//! 			if (target.hasType('Grass')) return false;
//! 			if (!target.addType('Grass')) return false;
//! 			this.add('-start', target, 'typeadd', 'Grass', '[from] move: Forest\'s Curse');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Grass",
//! 		zMove: { boost: { atk: 1, def: 1, spa: 1, spd: 1, spe: 1 } },
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

