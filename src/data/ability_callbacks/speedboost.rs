//! Speed Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.activeTurns) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Boost Speed by 1 stage if Pokemon has been active for at least 1 turn
    let active_turns = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.active_turns
    };

    if active_turns > 0 {
        battle.boost(&[("spe", 1)], pokemon_pos, None, None, false, false);
    }

    EventResult::Continue
}

