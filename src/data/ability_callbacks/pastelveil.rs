//! Pastel Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const ally of pokemon.alliesAndSelf()) {
///         if (['psn', 'tox'].includes(ally.status)) {
///             this.add('-activate', pokemon, 'ability: Pastel Veil');
///             ally.cureStatus();
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (['psn', 'tox'].includes(pokemon.status)) {
///         this.add('-activate', pokemon, 'ability: Pastel Veil');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAnySwitchIn() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (!['psn', 'tox'].includes(status.id)) return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Pastel Veil');
///     }
///     return false;
/// }
pub fn on_set_status(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllySetStatus(status, target, source, effect) {
///     if (!['psn', 'tox'].includes(status.id)) return;
///     if ((effect as Move)?.status) {
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Pastel Veil', `[of] ${effectHolder}`);
///     }
///     return false;
/// }
pub fn on_ally_set_status(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

