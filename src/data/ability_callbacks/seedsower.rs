//! Seed Sower Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     this.field.setTerrain('grassyterrain');
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // Set terrain to Grassy Terrain when hit by a damaging move
    battle.set_terrain(crate::ID::from("grassyterrain"), None);
    EventResult::Continue
}

