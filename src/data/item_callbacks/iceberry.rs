//! Ice Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	iceberry: {
//! 		name: "Ice Berry",
//! 		spritenum: 381,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 80,
//! 			type: "Grass",
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'brn') {
//! 				pokemon.eatItem();
//! 			}
//! 		},
//! 		onEat(pokemon) {
//! 			if (pokemon.status === 'brn') {
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		num: 152,
//! 		gen: 2,
//! 		isNonstandard: "Past",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
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
