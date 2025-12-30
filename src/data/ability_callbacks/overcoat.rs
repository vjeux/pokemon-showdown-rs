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
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
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
pub fn on_try_hit(_battle: &mut Battle, _target_pos: (usize, usize), _source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

