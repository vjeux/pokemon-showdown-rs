//! Suction Cups Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDragOut(pokemon) {
///     this.add('-activate', pokemon, 'ability: Suction Cups');
///     return null;
/// }
pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

