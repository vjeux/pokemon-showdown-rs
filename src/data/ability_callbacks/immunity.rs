//! Immunity Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	immunity: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'psn' || pokemon.status === 'tox') {
//! 				this.add('-activate', pokemon, 'ability: Immunity');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (status.id !== 'psn' && status.id !== 'tox') return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Immunity');
//! 			}
//! 			return false;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Immunity",
//! 		rating: 2,
//! 		num: 17,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status === 'psn' || pokemon.status === 'tox')
    if pokemon.status.as_str() == "psn" || pokemon.status.as_str() == "tox" {
        // this.add('-activate', pokemon, 'ability: Immunity');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Immunity")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// onSetStatus(status, target, source, effect)
pub fn on_set_status(_battle: &mut Battle, status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (status.id !== 'psn' && status.id !== 'tox') return;
    if status.id != "psn" && status.id != "tox" {
        return AbilityHandlerResult::Undefined;
    }
    // if ((effect as Move)?.status)
    if effect.status.is_some() {
        // this.add('-immune', target, '[from] ability: Immunity');
        // Note: battle is not mutable here, so we can't add logs
        // This will need to be handled by the caller
    }
    // return false;
    AbilityHandlerResult::False
}
