//! Arena Trap Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	arenatrap: {
//! 		onFoeTrapPokemon(pokemon) {
//! 			if (!pokemon.isAdjacent(this.effectState.target)) return;
//! 			if (pokemon.isGrounded()) {
//! 				pokemon.tryTrap(true);
//! 			}
//! 		},
//! 		onFoeMaybeTrapPokemon(pokemon, source) {
//! 			if (!source) source = this.effectState.target;
//! 			if (!source || !pokemon.isAdjacent(source)) return;
//! 			if (pokemon.isGrounded(!pokemon.knownType)) { // Negate immunity if the type is unknown
//! 				pokemon.maybeTrapped = true;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Arena Trap",
//! 		rating: 5,
//! 		num: 71,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onFoeTrapPokemon(pokemon)
pub fn on_foe_trap_pokemon(battle: &mut Battle, pokemon: &mut Pokemon, arena_trap_holder: &Pokemon) -> AbilityHandlerResult {
    // if (!pokemon.isAdjacent(this.effectState.target)) return;
    // Check if the trapped pokemon is adjacent to the Arena Trap holder
    let active_per_half = if battle.game_type == crate::dex_data::GameType::Singles { 1 } else { 2 };
    if !pokemon.is_adjacent(arena_trap_holder.position, arena_trap_holder.fainted, active_per_half) {
        return AbilityHandlerResult::Undefined;
    }

    // if (pokemon.isGrounded())
    if pokemon.is_grounded() {
        // pokemon.tryTrap(true);
        pokemon.try_trap(true);
    }
    AbilityHandlerResult::Undefined
}

/// onFoeMaybeTrapPokemon(pokemon, source)
pub fn on_foe_maybe_trap_pokemon(battle: &mut Battle, pokemon: &mut Pokemon, source: Option<&Pokemon>, arena_trap_holder: &Pokemon) -> AbilityHandlerResult {
    // if (!source) source = this.effectState.target;
    let source = source.unwrap_or(arena_trap_holder);

    // if (!source || !pokemon.isAdjacent(source)) return;
    let active_per_half = if battle.game_type == crate::dex_data::GameType::Singles { 1 } else { 2 };
    if !pokemon.is_adjacent(source.position, source.fainted, active_per_half) {
        return AbilityHandlerResult::Undefined;
    }

    // if (pokemon.isGrounded(!pokemon.knownType))
    // Note: We don't have knownType tracking yet, so we just use is_grounded()
    if pokemon.is_grounded() {
        // pokemon.maybeTrapped = true;
        pokemon.maybe_trapped = true;
    }
    AbilityHandlerResult::Undefined
}
