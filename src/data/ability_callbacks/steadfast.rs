//! Steadfast Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	steadfast: {
//! 		onFlinch(pokemon) {
//! 			this.boost({ spe: 1 });
//! 		},
//! 		flags: {},
//! 		name: "Steadfast",
//! 		rating: 1,
//! 		num: 80,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onFlinch(pokemon)
pub fn on_flinch(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    let pokemon_ref = (pokemon.side_index, pokemon.position);
    // this.boost({ spe: 1 });
    battle.boost(&[("spe", 1)], pokemon_ref, Some(pokemon_ref), None);
    AbilityHandlerResult::Undefined
}
