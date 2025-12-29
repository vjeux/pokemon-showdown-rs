//! Earth Eater Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Ground') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Earth Eater');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

