//! Regenerator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchOut(pokemon) {
///     pokemon.heal(pokemon.baseMaxhp / 3);
/// }
pub fn on_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

