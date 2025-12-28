//! Tar Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.terastallized) return false;
    ///     this.add('-start', pokemon, 'Tar Shot');
    /// }
    pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEffectiveness(typeMod, target, type, move) {
    ///     if (move.type !== 'Fire') return;
    ///     if (!target) return;
    ///     if (type !== target.getTypes()[0]) return;
    ///     return typeMod + 1;
    /// }
    pub fn on_effectiveness(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
