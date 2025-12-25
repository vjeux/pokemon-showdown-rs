//! Magma Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	magmaarmor: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'frz') {
//! 				this.add('-activate', pokemon, 'ability: Magma Armor');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'frz') return false;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Magma Armor",
//! 		rating: 0.5,
//! 		num: 40,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status === 'frz')
    if pokemon.status.as_str() == "frz" {
        // this.add('-activate', pokemon, 'ability: Magma Armor');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Magma Armor")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// onImmunity(type, pokemon)
pub fn on_immunity(immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'frz') return false;
    if immunity_type == "frz" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}
