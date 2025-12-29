//! Bad Dreams Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (!pokemon.hp) return;
///     for (const target of pokemon.foes()) {
///         if (target.status === 'slp' || target.hasAbility('comatose')) {
///             this.damage(target.baseMaxhp / 8, target, pokemon);
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

