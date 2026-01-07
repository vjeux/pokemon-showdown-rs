//! Psychic Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('psychicterrain');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Set terrain to Psychic Terrain
    let source_effect = Some(crate::battle::Effect::ability("psychicsurge"));
    battle.set_terrain(crate::ID::from("psychicterrain"), None, source_effect);
    EventResult::Continue
}

