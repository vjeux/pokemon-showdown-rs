//! Throat Chop Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'Throat Chop', '[silent]');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['sound']) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (!move.isZOrMaxPowered && move.flags['sound']) {
    ///         this.add('cant', pokemon, 'move: Throat Chop');
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (!move.isZOrMaxPowered && move.flags['sound']) {
    ///         this.add('cant', pokemon, 'move: Throat Chop');
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Throat Chop', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
