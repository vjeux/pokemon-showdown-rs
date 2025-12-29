//! Arena Trap Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFoeTrapPokemon(pokemon) {
///     if (!pokemon.isAdjacent(this.effectState.target)) return;
///     if (pokemon.isGrounded()) {
///         pokemon.tryTrap(true);
///     }
/// }
pub fn on_foe_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onFoeMaybeTrapPokemon(pokemon, source) {
///     if (!source) source = this.effectState.target;
///     if (!source || !pokemon.isAdjacent(source)) return;
///     if (pokemon.isGrounded(!pokemon.knownType)) { // Negate immunity if the type is unknown
///         pokemon.maybeTrapped = true;
///     }
/// }
pub fn on_foe_maybe_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

