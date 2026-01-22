//! Seed Sower Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     this.field.setTerrain('grassyterrain');
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Set terrain to Grassy Terrain when hit by a damaging move
    // The target is the Pokemon with this ability
    let source_effect = Some(crate::battle::Effect::ability("seedsower"));
    battle.set_terrain(crate::ID::from("grassyterrain"), target_pos, source_effect);
    EventResult::Continue
}

