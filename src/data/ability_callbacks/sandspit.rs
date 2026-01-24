//! Sand Spit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     this.field.setWeather('sandstorm');
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Set weather to Sandstorm when hit by a damaging move
    // Use battle.set_weather() instead of battle.set_weather() to get proper duration handling
    let sandspit_id = crate::ID::from("sandspit");
    battle.set_weather(crate::ID::from("sandstorm"), target_pos, Some(battle.make_ability_effect(&sandspit_id)));
    EventResult::Continue
}

