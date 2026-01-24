//! Misty Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('mistyterrain');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Set terrain to Misty Terrain
    let mistysurge_id = crate::ID::from("mistysurge");
    let source_effect = Some(battle.make_ability_effect(&mistysurge_id));
    battle.set_terrain(crate::ID::from("mistyterrain"), Some(pokemon_pos), source_effect);
    EventResult::Continue
}

