//! Core Enforcer Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getAbility().flags['cantsuppress']) return;
///     if (target.newlySwitched || this.queue.willMove(target)) return;
///     target.addVolatile('gastroacid');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target) {
///     if (target.getAbility().flags['cantsuppress']) return;
///     if (target.newlySwitched || this.queue.willMove(target)) return;
///     target.addVolatile('gastroacid');
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

