//! Minus Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySpA(spa, pokemon) {
///     for (const allyActive of pokemon.allies()) {
///         if (allyActive.hasAbility(['minus', 'plus'])) {
///             return this.chainModify(1.5);
///         }
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

