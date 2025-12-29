//! Safety Goggles Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, _pokemon_pos: (usize, usize), immunity_type: &str) -> EventResult {
    // if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
    if immunity_type == "sandstorm" || immunity_type == "hail" || immunity_type == "powder" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(pokemon, source, move) {
///     if (move.flags['powder'] && pokemon !== source && this.dex.getImmunity('powder', pokemon)) {
///         this.add('-activate', pokemon, 'item: Safety Goggles', move.name);
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
