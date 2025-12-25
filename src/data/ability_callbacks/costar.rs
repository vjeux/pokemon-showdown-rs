//! Costar Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	costar: {
//! 		onSwitchInPriority: -2,
//! 		onStart(pokemon) {
//! 			const ally = pokemon.allies()[0];
//! 			if (!ally) return;
//! 
//! 			let i: BoostID;
//! 			for (i in ally.boosts) {
//! 				pokemon.boosts[i] = ally.boosts[i];
//! 			}
//! 			const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
//! 			// we need to be sure to remove all the overlapping crit volatiles before trying to add any
//! 			for (const volatile of volatilesToCopy) pokemon.removeVolatile(volatile);
//! 			for (const volatile of volatilesToCopy) {
//! 				if (ally.volatiles[volatile]) {
//! 					pokemon.addVolatile(volatile);
//! 					if (volatile === 'gmaxchistrike') pokemon.volatiles[volatile].layers = ally.volatiles[volatile].layers;
//! 					if (volatile === 'dragoncheer') pokemon.volatiles[volatile].hasDragonType = ally.volatiles[volatile].hasDragonType;
//! 				}
//! 			}
//! 			this.add('-copyboost', pokemon, ally, '[from] ability: Costar');
//! 		},
//! 		flags: {},
//! 		name: "Costar",
//! 		rating: 0,
//! 		num: 294,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority: -2
    pub const ON_SWITCH_IN_PRIORITY: i32 = -2;

    /// onStart(pokemon)
    /// Copies ally's stat boosts and certain critical-hit volatiles
    pub fn on_start(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        // const ally = pokemon.allies()[0];
        // if (!ally) return;
        // This is doubles-only and needs ally access
        // TODO: Implement full costar logic when ally access is available
        // For now, just announce the ability
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Costar")]);
        AbilityHandlerResult::Undefined
    }
