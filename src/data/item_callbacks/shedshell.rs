//! Shed Shell Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::TrappedState;

/// onTrapPokemon(pokemon) {
///     pokemon.trapped = false;
/// }
pub fn on_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.trapped = false;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.trapped = TrappedState::None;
    }
    EventResult::Continue
}

/// onMaybeTrapPokemon(pokemon) {
///     pokemon.maybeTrapped = false;
/// }
pub fn on_maybe_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.maybeTrapped = false;
    if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        pokemon.maybe_trapped = false;
    }
    EventResult::Continue
}
