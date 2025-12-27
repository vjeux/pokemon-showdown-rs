//! Shell Trap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// priorityChargeCallback(pokemon) {
///     pokemon.addVolatile('shelltrap');
/// }
pub fn priority_charge_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryMove(pokemon) {
///     if (!pokemon.volatiles['shelltrap']?.gotHit) {
///         this.attrLastMove('[still]');
///         this.add('cant', pokemon, 'Shell Trap', 'Shell Trap');
///         return null;
///     }
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Shell Trap');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onHit(pokemon, source, move) {
    ///     if (!pokemon.isAlly(source) && move.category === 'Physical') {
    ///         this.effectState.gotHit = true;
    ///         const action = this.queue.willMove(pokemon);
    ///         if (action) {
    ///             this.queue.prioritizeAction(action);
    ///         }
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
