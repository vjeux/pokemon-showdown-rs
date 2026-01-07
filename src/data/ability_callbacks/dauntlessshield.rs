//! Dauntless Shield Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.shieldBoost) return;
///     pokemon.shieldBoost = true;
///     this.boost({ def: 1 }, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // if (pokemon.shieldBoost) return;
    let already_boosted = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.shield_boost.unwrap_or(false)
    };

    if already_boosted {
        return EventResult::Continue;
    }

    // pokemon.shieldBoost = true;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.shield_boost = Some(true);
    }

    // this.boost({ def: 1 }, pokemon);
    battle.boost(&[("def", 1)], pokemon_pos, Some(pokemon_pos), None, false, true);

    EventResult::Continue
}

