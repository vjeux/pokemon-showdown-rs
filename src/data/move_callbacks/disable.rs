//! Disable Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (!target.lastMove || target.lastMove.isZOrMaxPowered || target.lastMove.isMax || target.lastMove.id === 'struggle') {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     // The target hasn't taken its turn, or Cursed Body activated and the move was not used through Dancer or Instruct
    ///     if (
    ///         this.queue.willMove(pokemon) ||
    ///         (pokemon === this.activePokemon && this.activeMove && !this.activeMove.isExternal)
    ///     ) {
    ///         this.effectState.duration!--;
    ///     }
    ///     if (!pokemon.lastMove) {
    ///         this.debug(`Pokemon hasn't moved yet`);
    ///         return false;
    ///     }
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id === pokemon.lastMove.id) {
    ///             if (!moveSlot.pp) {
    ///                 this.debug('Move out of PP');
    ///                 return false;
    ///             }
    ///         }
    ///     }
    ///     if (effect.effectType === 'Ability') {
    ///         this.add('-start', pokemon, 'Disable', pokemon.lastMove.name, '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-start', pokemon, 'Disable', pokemon.lastMove.name);
    ///     }
    ///     this.effectState.move = pokemon.lastMove.id;
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Disable');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(attacker, defender, move) {
    ///     if (!(move.isZ && move.isZOrMaxPowered) && move.id === this.effectState.move) {
    ///         this.add('cant', attacker, 'Disable', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id === this.effectState.move) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
