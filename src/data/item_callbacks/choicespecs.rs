//! Choice Specs Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	choicespecs: {
//! 		name: "Choice Specs",
//! 		spritenum: 70,
//! 		fling: {
//! 			basePower: 10,
//! 		},
//! 		onStart(pokemon) {
//! 			if (pokemon.volatiles['choicelock']) {
//! 				this.debug('removing choicelock');
//! 			}
//! 			pokemon.removeVolatile('choicelock');
//! 		},
//! 		onModifyMove(move, pokemon) {
//! 			pokemon.addVolatile('choicelock');
//! 		},
//! 		onModifySpAPriority: 1,
//! 		onModifySpA(spa, pokemon) {
//! 			if (pokemon.volatiles['dynamax']) return;
//! 			return this.chainModify(1.5);
//! 		},
//! 		isChoice: true,
//! 		num: 297,
//! 		gen: 4,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(...)
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifySpAPriority(...)
pub fn on_modify_sp_a_priority(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifySpA(spa, pokemon)
pub fn on_modify_sp_a(battle: &Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // JS: if (pokemon.volatiles['dynamax']) return;
    if pokemon.volatiles.contains_key(&ID::new("dynamax")) {
        return EventResult::Continue;
    }
    // JS: return this.chainModify(1.5);
    EventResult::Modify(1.5)
}
