//! Trace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.effectState.seek = true;
///     // n.b. only affects Hackmons
///     // interaction with No Ability is complicated: https://www.smogon.com/forums/threads/pokemon-sun-moon-battle-mechanics-research.3586701/page-76#post-7790209
///     if (pokemon.adjacentFoes().some(foeActive => foeActive.ability === 'noability')) {
///         this.effectState.seek = false;
///     }
///     // interaction with Ability Shield is similar to No Ability
///     if (pokemon.hasItem('Ability Shield')) {
///         this.add('-block', pokemon, 'item: Ability Shield');
///         this.effectState.seek = false;
///     }
///     if (this.effectState.seek) {
///         this.singleEvent('Update', this.effect, this.effectState, pokemon);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (!this.effectState.seek) return;
/// 
///     const possibleTargets = pokemon.adjacentFoes().filter(
///         target => !target.getAbility().flags['notrace'] && target.ability !== 'noability'
///     );
///     if (!possibleTargets.length) return;
/// 
///     const target = this.sample(possibleTargets);
///     const ability = target.getAbility();
///     pokemon.setAbility(ability, target);
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

