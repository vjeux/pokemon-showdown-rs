//! Insomnia Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	insomnia: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'slp') {
//! 				this.add('-activate', pokemon, 'ability: Insomnia');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (status.id !== 'slp') return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Insomnia');
//! 			}
//! 			return false;
//! 		},
//! 		onTryAddVolatile(status, target) {
//! 			if (status.id === 'yawn') {
//! 				this.add('-immune', target, '[from] ability: Insomnia');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Insomnia",
//! 		rating: 1.5,
//! 		num: 15,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status === 'slp')
    if pokemon.status.as_str() == "slp" {
        // this.add('-activate', pokemon, 'ability: Insomnia');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Insomnia")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// onSetStatus(status, target, source, effect)
pub fn on_set_status(_battle: &mut Battle, status: &Status, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (status.id !== 'slp') return;
    if status.id != "slp" {
        return AbilityHandlerResult::Undefined;
    }
    // if ((effect as Move)?.status)
    if effect.status.is_some() {
        // this.add('-immune', target, '[from] ability: Insomnia');
        // Note: battle is not mutable here, so we can't add logs
        // This will need to be handled by the caller
    }
    // return false;
    AbilityHandlerResult::False
}

/// onTryAddVolatile(status, target)
pub fn on_try_add_volatile(battle: &mut Battle, status: &Status, target: &Pokemon) -> AbilityHandlerResult {
    // if (status.id === 'yawn')
    if status.id == "yawn" {
        // this.add('-immune', target, '[from] ability: Insomnia');
        battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Insomnia")]);
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}
