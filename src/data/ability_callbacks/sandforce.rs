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
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), move_id: &str) -> EventResult {
    if battle.is_weather("sandstorm") {
        if let Some(active_move) = battle.dex.get_active_move(move_id) {
            let move_type = active_move.move_type.as_str();
            if move_type == "Rock" || move_type == "Ground" || move_type == "Steel" {
                battle.chain_modify_fraction(5325, 4096); return EventResult::Continue;
            }
        }
    }
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

