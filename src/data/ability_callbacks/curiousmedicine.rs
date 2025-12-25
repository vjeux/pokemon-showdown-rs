//! Curious Medicine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	curiousmedicine: {
//! 		onStart(pokemon) {
//! 			for (const ally of pokemon.adjacentAllies()) {
//! 				ally.clearBoosts();
//! 				this.add('-clearboost', ally, '[from] ability: Curious Medicine', `[of] ${pokemon}`);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Curious Medicine",
//! 		rating: 0,
//! 		num: 261,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
    /// Clears all stat boosts from adjacent allies when switched in
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // for (const ally of pokemon.adjacentAllies())
        // This is doubles-only and requires ally access
        // TODO: Implement when adjacentAllies() is available
        // For now, just announce the ability
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Curious Medicine")]);
        AbilityHandlerResult::Undefined
    }
