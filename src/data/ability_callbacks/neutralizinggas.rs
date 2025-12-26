//! Neutralizing Gas Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	neutralizinggas: {
//! 		// Ability suppression implemented in sim/pokemon.ts:Pokemon#ignoringAbility
//! 		onSwitchInPriority: 2,
//! 		onSwitchIn(pokemon) {
//! 			this.add('-ability', pokemon, 'Neutralizing Gas');
//! 			pokemon.abilityState.ending = false;
//! 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
//! 			for (const target of this.getAllActive()) {
//! 				if (target.hasItem('Ability Shield')) {
//! 					this.add('-block', target, 'item: Ability Shield');
//! 					continue;
//! 				}
//! 				// Can't suppress a Tatsugiri inside of Dondozo already
//! 				if (target.volatiles['commanding']) {
//! 					continue;
//! 				}
//! 				if (target.illusion) {
//! 					this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, pokemon, 'neutralizinggas');
//! 				}
//! 				if (target.volatiles['slowstart']) {
//! 					delete target.volatiles['slowstart'];
//! 					this.add('-end', target, 'Slow Start', '[silent]');
//! 				}
//! 				if (strongWeathers.includes(target.getAbility().id)) {
//! 					this.singleEvent('End', this.dex.abilities.get(target.getAbility().id), target.abilityState, target, pokemon, 'neutralizinggas');
//! 				}
//! 			}
//! 		},
//! 		onEnd(source) {
//! 			if (source.transformed) return;
//! 			for (const pokemon of this.getAllActive()) {
//! 				if (pokemon !== source && pokemon.hasAbility('Neutralizing Gas')) {
//! 					return;
//! 				}
//! 			}
//! 			this.add('-end', source, 'ability: Neutralizing Gas');
//! 
//! 			// FIXME this happens before the pokemon switches out, should be the opposite order.
//! 			// Not an easy fix since we cant use a supported event. Would need some kind of special event that
//! 			// gathers events to run after the switch and then runs them when the ability is no longer accessible.
//! 			// (If you're tackling this, do note extreme weathers have the same issue)
//! 
//! 			// Mark this pokemon's ability as ending so Pokemon#ignoringAbility skips it
//! 			if (source.abilityState.ending) return;
//! 			source.abilityState.ending = true;
//! 			const sortedActive = this.getAllActive();
//! 			this.speedSort(sortedActive);
//! 			for (const pokemon of sortedActive) {
//! 				if (pokemon !== source) {
//! 					if (pokemon.getAbility().flags['cantsuppress']) continue; // does not interact with e.g Ice Face, Zen Mode
//! 					if (pokemon.hasItem('abilityshield')) continue; // don't restart abilities that weren't suppressed
//! 
//! 					// Will be suppressed by Pokemon#ignoringAbility if needed
//! 					this.singleEvent('Start', pokemon.getAbility(), pokemon.abilityState, pokemon);
//! 					if (pokemon.ability === "gluttony") {
//! 						pokemon.abilityState.gluttony = false;
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
//! 		name: "Neutralizing Gas",
//! 		rating: 3.5,
//! 		num: 256,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = 2;

/// onSwitchIn(pokemon)
/// Suppresses all abilities on the field (implemented in Pokemon#ignoringAbility)
///
/// TODO: onSwitchIn handler not yet implemented
/// TODO: Very complex - suppresses abilities field-wide
/// TODO: Needs pokemon.abilityState.ending, getAllActive(), hasItem('Ability Shield')
/// TODO: Needs volatiles['commanding'], illusion field, volatiles['slowstart']
/// TODO: Needs singleEvent system, getAbility()
/// When implemented, should:
/// 1. Add ability message
/// 2. Set abilityState.ending = false
/// 3. Loop through all active pokemon:
///    - Skip if has Ability Shield item
///    - Skip if has commanding volatile
///    - End Illusion if present
///    - Remove slowstart volatile
///    - End strong weather abilities (desolateland/primordialsea/deltastream)
pub fn on_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onEnd(source)
/// Re-activates suppressed abilities when Neutralizing Gas ends
///
/// TODO: onEnd handler not yet implemented
/// TODO: Needs source.transformed check, getAllActive(), speedSort()
/// TODO: Needs ability.flags['cantsuppress'], singleEvent('Start')
/// When implemented, should:
/// 1. Return if source.transformed
/// 2. Check if another Neutralizing Gas is still active
/// 3. Add end message
/// 4. Mark abilityState.ending = true
/// 5. Speed-sort all active pokemon
/// 6. Fire Start event for each pokemon's ability (except source and cantsuppress)
/// 7. Special handling for gluttony ability
pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

