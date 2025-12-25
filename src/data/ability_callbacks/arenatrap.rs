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
/// Note: Simplified implementation - isAdjacent and isGrounded checks not available
pub fn on_foe_trap_pokemon(_battle: &mut Battle, pokemon: &mut Pokemon, _arena_trap_holder: &Pokemon) -> AbilityHandlerResult {
    // if (!pokemon.isAdjacent(this.effectState.target)) return;
    // TODO: Need isAdjacent check

    // if (pokemon.isGrounded())
    // TODO: Need isGrounded check - for now assume grounded if not Flying type
    let is_grounded = !pokemon.types.contains(&"Flying".to_string());
    if is_grounded {
        // pokemon.tryTrap(true);
        pokemon.trapped = true;
    }
    AbilityHandlerResult::Undefined
}

/// onFoeMaybeTrapPokemon(pokemon, source)
pub fn on_foe_maybe_trap_pokemon(_battle: &mut Battle, pokemon: &mut Pokemon, _source: Option<&Pokemon>, _arena_trap_holder: &Pokemon) -> AbilityHandlerResult {
    // if (!source) source = this.effectState.target;
    // if (!source || !pokemon.isAdjacent(source)) return;
    // TODO: Need isAdjacent check

    // if (pokemon.isGrounded(!pokemon.knownType))
    // TODO: Need isGrounded check - for now assume grounded if not Flying type
    let is_grounded = !pokemon.types.contains(&"Flying".to_string());
    if is_grounded {
        // pokemon.maybeTrapped = true;
        pokemon.maybe_trapped = true;
    }
    AbilityHandlerResult::Undefined
}
