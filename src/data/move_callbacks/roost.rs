//! Roost Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (target.terastallized) {
    ///         if (target.hasType('Flying')) {
    ///             this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
    ///         }
    ///         return false;
    ///     }
    ///     this.add('-singleturn', target, 'move: Roost');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onType(types, pokemon) {
    ///     this.effectState.typeWas = types;
    ///     return types.filter(type => type !== 'Flying');
    /// }
    pub fn on_type(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
