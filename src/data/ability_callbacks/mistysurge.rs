//! Misty Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('mistyterrain');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Set terrain to Misty Terrain
    battle.set_terrain(crate::ID::from("mistyterrain"), None);
    EventResult::Continue
}

