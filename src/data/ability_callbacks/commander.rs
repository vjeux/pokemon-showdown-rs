//! Commander Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnySwitchIn() {
///     ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onStart(pokemon) {
///     ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (this.gameType !== 'doubles') return;
///     // don't run between when a Pokemon switches in and the resulting onSwitchIn event
///     if (this.queue.peek()?.choice === 'runSwitch') return;
/// 
///     const ally = pokemon.allies()[0];
///     if (pokemon.switchFlag || ally?.switchFlag) return;
///     if (!ally || pokemon.baseSpecies.baseSpecies !== 'Tatsugiri' || ally.baseSpecies.baseSpecies !== 'Dondozo') {
///         // Handle any edge cases
///         if (pokemon.getVolatile('commanding')) pokemon.removeVolatile('commanding');
///         return;
///     }
/// 
///     if (!pokemon.getVolatile('commanding')) {
///         // If Dondozo already was commanded this fails
///         if (ally.getVolatile('commanded')) return;
///         // Cancel all actions this turn for pokemon if applicable
///         this.queue.cancelAction(pokemon);
///         // Add volatiles to both pokemon
///         this.add('-activate', pokemon, 'ability: Commander', `[of] ${ally}`);
///         pokemon.addVolatile('commanding');
///         ally.addVolatile('commanded', pokemon);
///         // Continued in conditions.ts in the volatiles
///     } else {
///         if (!ally.fainted) return;
///         pokemon.removeVolatile('commanding');
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

