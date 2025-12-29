//! Good as Gold Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (move.category === 'Status' && target !== source) {
///         this.add('-immune', target, '[from] ability: Good as Gold');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

