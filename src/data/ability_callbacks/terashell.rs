//! Tera Shell Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyBeforeMove() {
///     delete this.effectState.resisted;
/// }
pub fn on_any_before_move(battle: &mut Battle) -> EventResult {
    // delete this.effectState.resisted;
    // In JS, `this.effectState` refers to the ability holder's abilityState
    // We need to use effect_state.target to find the Pokemon with Tera Shell
    // and clear their ability_state.resisted
    let effect_holder_pos = match battle.effect_state.borrow().target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if let Some(pokemon) = battle.pokemon_at_mut(effect_holder_pos.0, effect_holder_pos.1) {
        pokemon.ability_state.borrow_mut().resisted = None;
    }

    EventResult::Continue
}

/// onAnyAfterMove() {
///     delete this.effectState.resisted;
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // delete this.effectState.resisted;
    // In JS, `this.effectState` refers to the ability holder's abilityState
    // We need to use effect_state.target to find the Pokemon with Tera Shell
    // and clear their ability_state.resisted
    let effect_holder_pos = match battle.effect_state.borrow().target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if let Some(pokemon) = battle.pokemon_at_mut(effect_holder_pos.0, effect_holder_pos.1) {
        pokemon.ability_state.borrow_mut().resisted = None;
    }

    EventResult::Continue
}

