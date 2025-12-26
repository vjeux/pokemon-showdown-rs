//! Leftovers Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	leftovers: {
//! 		name: "Leftovers",
//! 		spritenum: 242,
//! 		fling: {
//! 			basePower: 10,
//! 		},
//! 		onResidualOrder: 5,
//! 		onResidualSubOrder: 4,
//! 		onResidual(pokemon) {
//! 			this.heal(pokemon.baseMaxhp / 16);
//! 		},
//! 		num: 234,
//! 		gen: 2,
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

    // JS: this.heal(pokemon.baseMaxhp / 16);
    let heal = pokemon.maxhp / 16;
    battle.heal(heal as i32, Some(pokemon_pos), None, Some(&ID::new("leftovers")));
    EventResult::Continue
}
