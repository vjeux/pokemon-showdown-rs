//! Electric Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('electricterrain');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // Set terrain to Electric Terrain
    battle.field.set_terrain(crate::ID::from("electricterrain"), None);
    EventResult::Continue
}

