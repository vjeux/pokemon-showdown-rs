//! Gorilla Tactics Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.abilityState.choiceLock = "";
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBeforeMove(pokemon, target, move) {
///     if (move.isZOrMaxPowered || move.id === 'struggle') return;
///     if (pokemon.abilityState.choiceLock && pokemon.abilityState.choiceLock !== move.id) {
///         // Fails unless ability is being ignored (these events will not run), no PP lost.
///         this.addMove('move', pokemon, move.name);
///         this.attrLastMove('[still]');
///         this.debug("Disabled by Gorilla Tactics");
///         this.add('-fail', pokemon);
///         return false;
///     }
/// }
pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.abilityState.choiceLock || move.isZOrMaxPowered || move.id === 'struggle') return;
///     pokemon.abilityState.choiceLock = move.id;
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.volatiles['dynamax']) return;
///     // PLACEHOLDER
///     this.debug('Gorilla Tactics Atk Boost');
///     return this.chainModify(1.5);
/// }
pub fn on_modify_atk(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onDisableMove(pokemon) {
///     if (!pokemon.abilityState.choiceLock) return;
///     if (pokemon.volatiles['dynamax']) return;
///     for (const moveSlot of pokemon.moveSlots) {
///         if (moveSlot.id !== pokemon.abilityState.choiceLock) {
///             pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
///         }
///     }
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEnd(pokemon) {
///     pokemon.abilityState.choiceLock = "";
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

