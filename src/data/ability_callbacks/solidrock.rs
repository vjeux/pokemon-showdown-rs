//! Solid Rock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (target.getMoveHitData(move).typeMod > 0) {
///         this.debug('Solid Rock neutralize');
///         return this.chainModify(0.75);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

