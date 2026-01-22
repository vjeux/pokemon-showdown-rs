//! Black Sludge Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.hasType('Poison')) {
///         this.heal(pokemon.baseMaxhp / 16);
///     } else {
///         this.damage(pokemon.baseMaxhp / 8);
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hasType('Poison'))
    let (has_poison, base_maxhp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.has_type(battle, "Poison"), pokemon.base_maxhp)
    };

    if has_poison {
        // this.heal(pokemon.baseMaxhp / 16);
        battle.heal(hp_fraction(base_maxhp, 16), Some(pokemon_pos), None, None);
    } else {
        // this.damage(pokemon.baseMaxhp / 8);
        battle.damage(hp_fraction(base_maxhp, 8), Some(pokemon_pos), None, None, false);
    }

    EventResult::Continue
}
