//! Bestow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	bestow: {
//! 		num: 516,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Bestow",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { mirror: 1, bypasssub: 1, allyanim: 1, noassist: 1, failcopycat: 1 },
//! 		onHit(target, source, move) {
//! 			if (target.item) {
//! 				return false;
//! 			}
//! 			const myItem = source.takeItem();
//! 			if (!myItem) return false;
//! 			if (!this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem) || !target.setItem(myItem)) {
//! 				source.item = myItem.id;
//! 				return false;
//! 			}
//! 			this.add('-item', target, myItem.name, '[from] move: Bestow', `[of] ${source}`);
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { boost: { spe: 2 } },
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

