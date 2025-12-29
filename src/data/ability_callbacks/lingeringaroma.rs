//! Lingering Aroma Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     const sourceAbility = source.getAbility();
///     if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'lingeringaroma') {
///         return;
///     }
///     if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
///         const oldAbility = source.setAbility('lingeringaroma', target);
///         if (oldAbility) {
///             this.add('-activate', target, 'ability: Lingering Aroma', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

