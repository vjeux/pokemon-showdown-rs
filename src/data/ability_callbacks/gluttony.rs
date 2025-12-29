//! Gluttony Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.abilityState.gluttony = true;
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onDamage(item, pokemon) {
///     pokemon.abilityState.gluttony = true;
/// }
pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

