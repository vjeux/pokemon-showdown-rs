//! Cheek Pouch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cheekpouch: {
//! 		onEatItem(item, pokemon) {
//! 			this.heal(pokemon.baseMaxhp / 3);
//! 		},
//! 		flags: {},
//! 		name: "Cheek Pouch",
//! 		rating: 2,
//! 		num: 167,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onEatItem(item, pokemon)
pub fn on_eat_item(battle: &mut Battle, _item_id: &str, pokemon: &Pokemon) -> AbilityHandlerResult {
    let pokemon_ref = (pokemon.side_index, pokemon.position);
    // this.heal(pokemon.baseMaxhp / 3);
    let heal_amount = pokemon.base_maxhp / 3;
    battle.heal(heal_amount as i32, Some(pokemon_ref), Some(pokemon_ref), None);
    AbilityHandlerResult::Undefined
}
