//! Dry Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Dry Skin');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(_battle: &mut Battle, _target_pos: (usize, usize), _source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSourceBasePower(basePower, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(1.25);
///     }
/// }
pub fn on_source_base_power(battle: &mut Battle, _base_power: i32, move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            let modified = battle.chain_modify(1.25);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

/// onWeather(target, source, effect) {
///     if (target.hasItem('utilityumbrella')) return;
///     if (effect.id === 'raindance' || effect.id === 'primordialsea') {
///         this.heal(target.baseMaxhp / 8);
///     } else if (effect.id === 'sunnyday' || effect.id === 'desolateland') {
///         this.damage(target.baseMaxhp / 8, target, target);
///     }
/// }
pub fn on_weather(_battle: &mut Battle, _weather_id: &str, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

