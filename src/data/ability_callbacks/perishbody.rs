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
pub fn on_damaging_hit(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

