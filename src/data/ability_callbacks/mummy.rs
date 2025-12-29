//! Mummy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     const sourceAbility = source.getAbility();
///     if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'mummy') {
///         return;
///     }
///     if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
///         const oldAbility = source.setAbility('mummy', target);
///         if (oldAbility) {
///             this.add('-activate', target, 'ability: Mummy', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

