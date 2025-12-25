//! Commander Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	commander: {
//! 		onAnySwitchInPriority: -2,
//! 		onAnySwitchIn() {
//! 			((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, this.effectState.target);
//! 		},
//! 		onStart(pokemon) {
//! 			((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (this.gameType !== 'doubles') return;
//! 			// don't run between when a Pokemon switches in and the resulting onSwitchIn event
//! 			if (this.queue.peek()?.choice === 'runSwitch') return;
//! 
//! 			const ally = pokemon.allies()[0];
//! 			if (pokemon.switchFlag || ally?.switchFlag) return;
//! 			if (!ally || pokemon.baseSpecies.baseSpecies !== 'Tatsugiri' || ally.baseSpecies.baseSpecies !== 'Dondozo') {
//! 				// Handle any edge cases
//! 				if (pokemon.getVolatile('commanding')) pokemon.removeVolatile('commanding');
//! 				return;
//! 			}
//! 
//! 			if (!pokemon.getVolatile('commanding')) {
//! 				// If Dondozo already was commanded this fails
//! 				if (ally.getVolatile('commanded')) return;
//! 				// Cancel all actions this turn for pokemon if applicable
//! 				this.queue.cancelAction(pokemon);
//! 				// Add volatiles to both pokemon
//! 				this.add('-activate', pokemon, 'ability: Commander', `[of] ${ally}`);
//! 				pokemon.addVolatile('commanding');
//! 				ally.addVolatile('commanded', pokemon);
//! 				// Continued in conditions.ts in the volatiles
//! 			} else {
//! 				if (!ally.fainted) return;
//! 				pokemon.removeVolatile('commanding');
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1 },
//! 		name: "Commander",
//! 		rating: 0,
//! 		num: 279,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Priority for onAnySwitchIn
    pub const ON_ANY_SWITCH_IN_PRIORITY: i32 = -2;

    /// onAnySwitchIn() - triggers onUpdate
    pub fn on_any_switch_in(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, this.effectState.target);
        on_update(battle, pokemon);
        AbilityHandlerResult::Undefined
    }

    /// onStart(pokemon) - triggers onUpdate
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
        on_update(battle, pokemon);
        AbilityHandlerResult::Undefined
    }

    /// onUpdate(pokemon) - main Commander logic
    /// This is a complex doubles-only ability for Tatsugiri + Dondozo
    /// Simplified implementation - full logic requires queue and volatile management
    pub fn on_update(_battle: &mut Battle, _pokemon: &Pokemon) -> AbilityHandlerResult {
        // TODO: Implement full Commander logic
        // Requires: get_ally(), cancel_action(), add/remove_volatile on battle, game_type check
        // This ability allows Tatsugiri to enter Dondozo's mouth in doubles battles
        AbilityHandlerResult::Undefined
    }
