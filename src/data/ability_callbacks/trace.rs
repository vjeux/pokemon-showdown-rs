//! Trace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	trace: {
//! 		onStart(pokemon) {
//! 			this.effectState.seek = true;
//! 			// n.b. only affects Hackmons
//! 			// interaction with No Ability is complicated: https://www.smogon.com/forums/threads/pokemon-sun-moon-battle-mechanics-research.3586701/page-76#post-7790209
//! 			if (pokemon.adjacentFoes().some(foeActive => foeActive.ability === 'noability')) {
//! 				this.effectState.seek = false;
//! 			}
//! 			// interaction with Ability Shield is similar to No Ability
//! 			if (pokemon.hasItem('Ability Shield')) {
//! 				this.add('-block', pokemon, 'item: Ability Shield');
//! 				this.effectState.seek = false;
//! 			}
//! 			if (this.effectState.seek) {
//! 				this.singleEvent('Update', this.effect, this.effectState, pokemon);
//! 			}
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (!this.effectState.seek) return;
//! 
//! 			const possibleTargets = pokemon.adjacentFoes().filter(
//! 				target => !target.getAbility().flags['notrace'] && target.ability !== 'noability'
//! 			);
//! 			if (!possibleTargets.length) return;
//! 
//! 			const target = this.sample(possibleTargets);
//! 			const ability = target.getAbility();
//! 			pokemon.setAbility(ability, target);
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
//! 		name: "Trace",
//! 		rating: 2.5,
//! 		num: 36,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onUpdate(...)
pub fn on_update(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

