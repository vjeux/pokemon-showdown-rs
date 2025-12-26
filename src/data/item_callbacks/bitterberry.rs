//! Bitter Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	bitterberry: {
//! 		name: "Bitter Berry",
//! 		spritenum: 334,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 80,
//! 			type: "Ground",
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (pokemon.volatiles['confusion']) {
//! 				pokemon.eatItem();
//! 			}
//! 		},
//! 		onEat(pokemon) {
//! 			pokemon.removeVolatile('confusion');
//! 		},
//! 		num: 156,
//! 		gen: 2,
//! 		isNonstandard: "Past",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onUpdate(...)
pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}

/// onEat(...)
pub fn on_eat(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
