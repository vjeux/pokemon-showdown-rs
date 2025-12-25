//! Intrepid Sword Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	intrepidsword: {
//! 		onStart(pokemon) {
//! 			if (pokemon.swordBoost) return;
//! 			pokemon.swordBoost = true;
//! 			this.boost({ atk: 1 }, pokemon);
//! 		},
//! 		flags: {},
//! 		name: "Intrepid Sword",
//! 		rating: 4,
//! 		num: 234,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Boosts Attack by 1 stage on first switch-in only
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    let pokemon_side = pokemon.side_index;
    let pokemon_idx = pokemon.position;

    // if (pokemon.swordBoost) return;
    if pokemon.sword_boost {
        return AbilityHandlerResult::Undefined;
    }

    // pokemon.swordBoost = true;
    // Need to set the flag before boosting
    if let Some(side) = battle.sides.get_mut(pokemon_side) {
        if let Some(poke) = side.pokemon.get_mut(pokemon_idx) {
            poke.sword_boost = true;
        }
    }

    // this.boost({ atk: 1 }, pokemon);
    battle.boost(&[("atk", 1)], (pokemon_side, pokemon_idx), Some((pokemon_side, pokemon_idx)), None);

    AbilityHandlerResult::Undefined
}

