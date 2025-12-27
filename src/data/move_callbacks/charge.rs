//! Charge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
    ///         this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
    ///     } else {
    ///         this.add('-start', pokemon, 'Charge');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onRestart(pokemon, source, effect) {
    ///     if (effect && ['Electromorphosis', 'Wind Power'].includes(effect.name)) {
    ///         this.add('-start', pokemon, 'Charge', this.activeMove!.name, '[from] ability: ' + effect.name);
    ///     } else {
    ///         this.add('-start', pokemon, 'Charge');
    ///     }
    /// }
    pub fn on_restart(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Electric') {
    ///         this.debug('charge boost');
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onMoveAborted(pokemon, target, move) {
    ///     if (move.type === 'Electric' && move.id !== 'charge') {
    ///         pokemon.removeVolatile('charge');
    ///     }
    /// }
    pub fn on_move_aborted(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onAfterMove(pokemon, target, move) {
    ///     if (move.type === 'Electric' && move.id !== 'charge') {
    ///         pokemon.removeVolatile('charge');
    ///     }
    /// }
    pub fn on_after_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Charge', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
