//! Encore Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     let move: Move | ActiveMove | null = target.lastMove;
    ///     if (!move || target.volatiles['dynamax']) return false;
    /// 
    ///     // Encore only works on Max Moves if the base move is not itself a Max Move
    ///     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    ///     const moveSlot = target.getMoveData(move.id);
    ///     if (move.isZ || move.isMax || move.flags['failencore'] || !moveSlot || moveSlot.pp <= 0) {
    ///         // it failed
    ///         return false;
    ///     }
    ///     this.effectState.move = move.id;
    ///     this.add('-start', target, 'Encore');
    ///     if (!this.queue.willMove(target)) {
    ///         this.effectState.duration!++;
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onOverrideAction(pokemon, target, move) {
    ///     if (move.id !== this.effectState.move) return this.effectState.move;
    /// }
    pub fn on_override_action(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(target) {
    ///     const moveSlot = target.getMoveData(this.effectState.move);
    ///     if (!moveSlot || moveSlot.pp <= 0) {
    ///         // early termination if you run out of PP
    ///         target.removeVolatile('encore');
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Encore');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     if (!this.effectState.move || !pokemon.hasMove(this.effectState.move)) {
    ///         return;
    ///     }
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id !== this.effectState.move) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
