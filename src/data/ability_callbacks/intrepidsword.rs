//! Intrepid Sword Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onStart(pokemon) {
///     if (pokemon.swordBoost) return;
///     pokemon.swordBoost = true;
///     this.boost({ atk: 1 }, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Check if boost was already applied
    let already_boosted = if let Some(side) = battle.sides.get(pokemon_pos.0) {
        if let Some(pokemon) = side.pokemon.get(pokemon_pos.1) {
            pokemon.volatiles.contains_key(&ID::from("swordboost"))
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    if already_boosted {
        return EventResult::Continue;
    }

    // Mark as boosted
    if let Some(side) = battle.sides.get_mut(pokemon_pos.0) {
        if let Some(pokemon) = side.pokemon.get_mut(pokemon_pos.1) {
            pokemon.volatiles.insert(ID::from("swordboost"), Default::default());
        }
    }

    // Apply +1 Attack boost
    // JS: this.boost({ atk: 1 }, pokemon);
    battle.boost(&[("atk", 1)], pokemon_pos, None, Some("intrepidsword"), false, false);

    EventResult::Continue
}
