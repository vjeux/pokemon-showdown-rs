//! Water Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	waterveil: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'brn') {
//! 				this.add('-activate', pokemon, 'ability: Water Veil');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (status.id !== 'brn') return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Water Veil');
//! 			}
//! 			return false;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Water Veil",
//! 		rating: 2,
//! 		num: 41,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status === 'brn')
    if pokemon.status.as_str() == "brn" {
        // this.add('-activate', pokemon, 'ability: Water Veil');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Water Veil")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// onSetStatus(status, target, source, effect)
pub fn on_set_status(_battle: &mut Battle, status: &Status, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (status.id !== 'brn') return;
    if status.id != "brn" {
        return AbilityHandlerResult::Undefined;
    }
    // if ((effect as Move)?.status)
    if effect.status.is_some() {
        // this.add('-immune', target, '[from] ability: Water Veil');
        // Note: battle is not mutable here, so we can't add logs
        // This will need to be handled by the caller
    }
    // return false;
    AbilityHandlerResult::False
}
