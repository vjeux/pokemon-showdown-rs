//! Floral Healing Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     let success = false;
///     if (this.field.isTerrain('grassyterrain')) {
///         success = !!this.heal(this.modify(target.baseMaxhp, 0.667));
///     } else {
///         success = !!this.heal(Math.ceil(target.baseMaxhp * 0.5));
///     }
///     if (success && !target.isAlly(source)) {
///         target.staleness = 'external';
///     }
///     if (!success) {
///         this.add('-fail', target, 'heal');
///         return this.NOT_FAIL;
///     }
///     return success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

