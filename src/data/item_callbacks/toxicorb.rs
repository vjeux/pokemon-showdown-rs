//! Toxic Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     pokemon.trySetStatus('tox', pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    eprintln!("[TOXIC_ORB DEBUG] on_residual called for Pokemon at {:?}", pokemon_pos);

    // pokemon.trySetStatus('tox', pokemon);
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    eprintln!("[TOXIC_ORB DEBUG] Pokemon {} status before: {}", pokemon_mut.name, pokemon_mut.status);
    pokemon_mut.try_set_status(crate::dex_data::ID::new("tox"), None);
    eprintln!("[TOXIC_ORB DEBUG] Pokemon {} status after: {}", pokemon_mut.name, pokemon_mut.status);

    EventResult::Continue
}
