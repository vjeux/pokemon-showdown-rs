//! Overcoat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
/// }
pub fn on_immunity(battle: &mut Battle, type_or_status: &str, pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "sandstorm" || type_or_status == "hail" || type_or_status == "powder" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (move.flags['powder'] && target !== source && this.dex.getImmunity('powder', target)) {
///         this.add('-immune', target, '[from] ability: Overcoat');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

