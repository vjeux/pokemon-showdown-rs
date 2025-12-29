//! Cotton Down Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     let activated = false;
///     for (const pokemon of this.getAllActive()) {
///         if (pokemon === target || pokemon.fainted) continue;
///         if (!activated) {
///             this.add('-ability', target, 'Cotton Down');
///             activated = true;
///         }
///         this.boost({ spe: -1 }, pokemon, target, null, true);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

