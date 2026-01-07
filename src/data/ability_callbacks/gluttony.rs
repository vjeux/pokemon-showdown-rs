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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon_mut.ability_state.set_bool("gluttony", true);

    EventResult::Continue
}

/// onDamage(item, pokemon) {
///     pokemon.abilityState.gluttony = true;
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon_mut.ability_state.set_bool("gluttony", true);

    EventResult::Continue
}

