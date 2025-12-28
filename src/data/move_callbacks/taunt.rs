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
    pub fn on_start(_battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'move: Taunt');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'move: Taunt');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("move: Taunt"),
            ],
        );

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
    pub fn on_disable_move(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(attacker, defender, move) {
    ///     if (!(move.isZ && move.isZOrMaxPowered) && move.category === 'Status' && move.id !== 'mefirst') {
    ///         this.add('cant', attacker, 'move: Taunt', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(_battle: &mut Battle, _move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
