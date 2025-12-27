//! Taunt Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (target.activeTurns && !this.queue.willMove(target)) {
    ///         this.effectState.duration!++;
    ///     }
    ///     this.add('-start', target, 'move: Taunt');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'move: Taunt');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         const move = this.dex.moves.get(moveSlot.id);
    ///         if (move.category === 'Status' && move.id !== 'mefirst') {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(attacker, defender, move) {
    ///     if (!(move.isZ && move.isZOrMaxPowered) && move.category === 'Status' && move.id !== 'mefirst') {
    ///         this.add('cant', attacker, 'move: Taunt', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
