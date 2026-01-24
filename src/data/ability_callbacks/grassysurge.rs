//! Grassy Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('grassyterrain');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Set terrain to Grassy Terrain
    let grassysurge_id = crate::ID::from("grassysurge");
    let source_effect = Some(battle.make_ability_effect(&grassysurge_id));
    battle.set_terrain(crate::ID::from("grassyterrain"), Some(pokemon_pos), source_effect);
    EventResult::Continue
}

