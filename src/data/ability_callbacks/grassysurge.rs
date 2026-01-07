//! Grassy Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('grassyterrain');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Set terrain to Grassy Terrain
    let source_effect = Some(crate::battle::Effect::ability("grassysurge"));
    battle.set_terrain(crate::ID::from("grassyterrain"), None, source_effect);
    EventResult::Continue
}

