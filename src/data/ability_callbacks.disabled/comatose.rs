//! Comatose Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	comatose: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Comatose');
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Comatose');
//! 			}
//! 			return false;
//! 		},
//! 		// Permanent sleep "status" implemented in the relevant sleep-checking effects
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Comatose",
//! 		rating: 4,
//! 		num: 213,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // this.add('-ability', pokemon, 'Comatose');
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Comatose")]);
        AbilityHandlerResult::Undefined
    }

    /// onSetStatus(status, target, source, effect)
    /// Blocks all status conditions
    pub fn on_set_status(battle: &mut Battle, _status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
        // if ((effect as Move)?.status)
        if effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Comatose');
            battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Comatose")]);
        }
        // return false;
        AbilityHandlerResult::False
    }
