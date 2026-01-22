//! Electric Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setTerrain('electricterrain');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Set terrain to Electric Terrain
    let source_effect = Some(crate::battle::Effect::ability("electricsurge"));
    battle.set_terrain(crate::ID::from("electricterrain"), Some(pokemon_pos), source_effect);
    EventResult::Continue
}

