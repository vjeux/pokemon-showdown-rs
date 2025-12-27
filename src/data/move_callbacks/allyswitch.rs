//! Ally Switch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return pokemon.addVolatile('allyswitch');
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(pokemon) {
///     let success = true;
///     // Fail in formats where you don't control allies
///     if (this.format.gameType !== 'doubles' && this.format.gameType !== 'triples') success = false;
/// 
///     // Fail in triples if the Pokemon is in the middle
///     if (pokemon.side.active.length === 3 && pokemon.position === 1) success = false;
/// 
///     const newPosition = (pokemon.position === 0 ? pokemon.side.active.length - 1 : 0);
///     if (!pokemon.side.active[newPosition]) success = false;
///     if (pokemon.side.active[newPosition].fainted) success = false;
///     if (!success) {
///         this.add('-fail', pokemon, 'move: Ally Switch');
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     this.swapPosition(pokemon, newPosition, '[from] move: Ally Switch');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.counter = 3;
    /// }
    pub fn on_start(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onRestart(pokemon) {
    ///     // this.effectState.counter should never be undefined here.
    ///     // However, just in case, use 1 if it is undefined.
    ///     const counter = this.effectState.counter || 1;
    ///     this.debug(`Ally Switch success chance: ${Math.round(100 / counter)}%`);
    ///     const success = this.randomChance(1, counter);
    ///     if (!success) {
    ///         delete pokemon.volatiles['allyswitch'];
    ///         return false;
    ///     }
    ///     if (this.effectState.counter < (this.effect as Condition).counterMax!) {
    ///         this.effectState.counter *= 3;
    ///     }
    ///     this.effectState.duration = 2;
    /// }
    pub fn on_restart(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
