//! Steadfast Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFlinch(pokemon) {
///     this.boost({ spe: 1 });
/// }
pub fn on_flinch(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Boost Speed by 1 stage when Pokemon flinches
    battle.boost(&[("spe", 1)], pokemon_pos, None, None, false, false);
    EventResult::Continue
}

