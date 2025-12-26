//! Black Sludge Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	blacksludge: {
//! 		name: "Black Sludge",
//! 		spritenum: 34,
//! 		fling: {
//! 			basePower: 30,
//! 		},
//! 		onResidualOrder: 5,
//! 		onResidualSubOrder: 4,
//! 		onResidual(pokemon) {
//! 			if (pokemon.hasType('Poison')) {
//! 				this.heal(pokemon.baseMaxhp / 16);
//! 			} else {
//! 				this.damage(pokemon.baseMaxhp / 8);
//! 			}
//! 		},
//! 		num: 281,
//! 		gen: 4,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onResidualOrder(...)
pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onResidualSubOrder(...)
pub fn on_residual_sub_order(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onResidual(pokemon)
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // JS: if (pokemon.hasType('Poison'))
    if pokemon.has_type("Poison") {
        // JS: this.heal(pokemon.baseMaxhp / 16);
        let heal = pokemon.maxhp / 16;
        battle.heal(heal as i32, Some(pokemon_pos), None, Some(&ID::new("blacksludge")));
    } else {
        // JS: this.damage(pokemon.baseMaxhp / 8);
        let damage = pokemon.maxhp / 8;
        battle.damage(damage as i32, Some(pokemon_pos), None, Some(&ID::new("blacksludge")), false);
    }
    EventResult::Continue
}
