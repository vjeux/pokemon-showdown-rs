//! Illusion Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	illusion: {
//! 		onBeforeSwitchIn(pokemon) {
//! 			pokemon.illusion = null;
//! 			// yes, you can Illusion an active pokemon but only if it's to your right
//! 			for (let i = pokemon.side.pokemon.length - 1; i > pokemon.position; i--) {
//! 				const possibleTarget = pokemon.side.pokemon[i];
//! 				if (!possibleTarget.fainted) {
//! 					// If Ogerpon is in the last slot while the Illusion Pokemon is Terastallized
//! 					// Illusion will not disguise as anything
//! 					if (!pokemon.terastallized || !['Ogerpon', 'Terapagos'].includes(possibleTarget.species.baseSpecies)) {
//! 						pokemon.illusion = possibleTarget;
//! 					}
//! 					break;
//! 				}
//! 			}
//! 		},
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (target.illusion) {
//! 				this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, source, move);
//! 			}
//! 		},
//! 		onEnd(pokemon) {
//! 			if (pokemon.illusion) {
//! 				this.debug('illusion cleared');
//! 				pokemon.illusion = null;
//! 				const details = pokemon.getUpdatedDetails();
//! 				this.add('replace', pokemon, details);
//! 				this.add('-end', pokemon, 'Illusion');
//! 				if (this.ruleTable.has('illusionlevelmod')) {
//! 					this.hint("Illusion Level Mod is active, so this Pok\u00e9mon's true level was hidden.", true);
//! 				}
//! 			}
//! 		},
//! 		onFaint(pokemon) {
//! 			pokemon.illusion = null;
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1 },
//! 		name: "Illusion",
//! 		rating: 4.5,
//! 		num: 149,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onBeforeSwitchIn(...)
pub fn on_before_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onDamagingHit(...)
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onEnd(...)
pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onFaint(...)
pub fn on_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

