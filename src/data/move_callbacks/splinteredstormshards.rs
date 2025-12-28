//! Splintered Stormshards Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit() {
///     this.field.clearTerrain();
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onHit() {
    //     this.field.clearTerrain();
    // }

    // this.field.clearTerrain();
    battle.field.clear_terrain();

    EventResult::Continue
}

/// onAfterSubDamage() {
///     this.field.clearTerrain();
/// }
pub fn on_after_sub_damage(battle: &mut Battle) -> EventResult {
    // onAfterSubDamage() {
    //     this.field.clearTerrain();
    // }

    // this.field.clearTerrain();
    battle.field.clear_terrain();

    EventResult::Continue
}

