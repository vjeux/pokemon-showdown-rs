//! Custap Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	custapberry: {
//! 		name: "Custap Berry",
//! 		spritenum: 86,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 100,
//! 			type: "Ghost",
//! 		},
//! 		onFractionalPriorityPriority: -2,
//! 		onFractionalPriority(priority, pokemon) {
//! 			if (
//! 				priority <= 0 &&
//! 				(pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
//! 					pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony))
//! 			) {
//! 				if (pokemon.eatItem()) {
//! 					this.add('-activate', pokemon, 'item: Custap Berry', '[consumed]');
//! 					return 0.1;
//! 				}
//! 			}
//! 		},
//! 		onEat() { },
//! 		num: 210,
//! 		gen: 4,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::ItemHandlerResult;

/// onFractionalPriorityPriority(...)
pub fn on_fractional_priority_priority(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}

/// onFractionalPriority(...)
pub fn on_fractional_priority(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}

/// onEat(...)
pub fn on_eat(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
