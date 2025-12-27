//! Fury Cutter Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (!pokemon.volatiles['furycutter'] || move.hit === 1) {
///         pokemon.addVolatile('furycutter');
///     }
///     const bp = this.clampIntRange(move.basePower * pokemon.volatiles['furycutter'].multiplier, 1, 160);
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.multiplier = 1;
    /// }
    pub fn on_start(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onRestart() {
    ///     if (this.effectState.multiplier < 4) {
    ///         this.effectState.multiplier <<= 1;
    ///     }
    ///     this.effectState.duration = 2;
    /// }
    pub fn on_restart(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
