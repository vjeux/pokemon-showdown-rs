//! Shed Tail Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(source) {
///     if (!this.canSwitch(source.side) || source.volatiles['commanded']) {
///         this.add('-fail', source);
///         return this.NOT_FAIL;
///     }
///     if (source.volatiles['substitute']) {
///         this.add('-fail', source, 'move: Shed Tail');
///         return this.NOT_FAIL;
///     }
///     if (source.hp <= Math.ceil(source.maxhp / 2)) {
///         this.add('-fail', source, 'move: Shed Tail', '[weak]');
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(target) {
///     this.directDamage(Math.ceil(target.maxhp / 2));
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

