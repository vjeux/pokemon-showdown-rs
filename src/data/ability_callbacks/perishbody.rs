//! Perish Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
///     this.add('-ability', target, 'Perish Body');
///     source.addVolatile('perishsong');
///     target.addVolatile('perishsong');
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

