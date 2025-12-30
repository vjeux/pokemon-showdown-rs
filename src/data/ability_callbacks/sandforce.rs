//! Sand Force Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (this.field.isWeather('sandstorm')) {
///         if (move.type === 'Rock' || move.type === 'Ground' || move.type === 'Steel') {
///             this.debug('Sand Force boost');
///             return this.chainModify([5325, 4096]);
///         }
///     }
/// }
pub fn on_base_power(_battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "sandstorm" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

