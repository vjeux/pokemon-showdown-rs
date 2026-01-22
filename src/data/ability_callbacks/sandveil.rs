//! Sand Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "sandstorm" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onModifyAccuracy(accuracy) {
///     if (typeof accuracy !== 'number') return;
///     if (this.field.isWeather('sandstorm')) {
///         this.debug('Sand Veil - decreasing accuracy');
///         return this.chainModify([3277, 4096]);
///     }
/// }
pub fn on_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    if battle.is_weather("sandstorm") {
        battle.chain_modify_fraction(3277, 4096); return EventResult::Continue;
    }
    EventResult::Continue
}

