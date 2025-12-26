//! Power Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	powerherb: {
//! 		onChargeMove(pokemon, target, move) {
//! 			if (pokemon.useItem()) {
//! 				this.debug('power herb - remove charge turn for ' + move.id);
//! 				this.attrLastMove('[still]');
//! 				this.addMove('-anim', pokemon, move.name, target);
//! 				return false; // skip charge turn
//! 			}
//! 		},
//! 		name: "Power Herb",
//! 		spritenum: 358,
//! 		fling: {
//! 			basePower: 10,
//! 		},
//! 		num: 271,
//! 		gen: 4,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onChargeMove(...)
pub fn on_charge_move(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
