//! Leftovers Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     this.heal(pokemon.baseMaxhp / 16);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon's base_maxhp
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // this.heal(pokemon.baseMaxhp / 16);
    battle.heal(base_maxhp / 16, Some(pokemon_pos), None, None);

    EventResult::Continue
}
