//! Toxic Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onResidual(pokemon) {
///     pokemon.trySetStatus('tox', pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    eprintln!("[TOXIC_ORB DEBUG] on_residual called for Pokemon at {:?}", pokemon_pos);

    // Get pokemon name and status for logging
    let (name, status_before) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.name.clone(), pokemon.status.clone())
    };

    eprintln!("[TOXIC_ORB DEBUG] Pokemon {} status before: {}", name, status_before);

    // pokemon.trySetStatus('tox', pokemon);
    Pokemon::try_set_status(battle, pokemon_pos, crate::dex_data::ID::new("tox"), None, None);

    let status_after = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.clone()
    };

    eprintln!("[TOXIC_ORB DEBUG] Pokemon {} status after: {}", name, status_after);

    EventResult::Continue
}
