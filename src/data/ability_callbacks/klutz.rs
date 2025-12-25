//! Klutz Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	klutz: {
//! 		// Klutz isn't technically active immediately in-game, but it activates early enough to beat all items
//! 		// we should keep an eye out in future gens for items that activate on switch-in before Unnerve
//! 		onSwitchInPriority: 1,
//! 		// Item suppression implemented in Pokemon.ignoringItem() within sim/pokemon.js
//! 		onStart(pokemon) {
//! 			this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
//! 		},
//! 		flags: {},
//! 		name: "Klutz",
//! 		rating: -1,
//! 		num: 103,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority: 1
pub const ON_SWITCH_IN_PRIORITY: i32 = 1;

/// onStart(pokemon)
/// Suppresses item - main logic in Pokemon.ignoringItem()
///
/// Note: Main item suppression logic should be in Pokemon.ignoringItem() method
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    let item_id = pokemon.get_item();
    let pokemon_loc = (pokemon.side_index, pokemon.position);

    battle.single_event(
        "End",
        item_id,
        Some(pokemon_loc),
        Some(pokemon_loc),
        None
    );

    AbilityHandlerResult::Undefined
}

