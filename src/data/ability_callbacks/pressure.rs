//! Pressure Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	pressure: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Pressure');
//! 		},
//! 		onDeductPP(target, source) {
//! 			if (target.isAlly(source)) return;
//! 			return 1;
//! 		},
//! 		flags: {},
//! 		name: "Pressure",
//! 		rating: 2.5,
//! 		num: 46,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-ability', pokemon, 'Pressure');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Pressure")]);
    AbilityHandlerResult::Undefined
}

/// onDeductPP(target, source)
pub fn on_deduct_p_p(_battle: &mut Battle, target: &Pokemon, source: &Pokemon) -> AbilityHandlerResult {
    // if (target.isAlly(source)) return;
    if target.side_index == source.side_index {
        return AbilityHandlerResult::Undefined;
    }
    // return 1;
    // Note: This should return 1 to deduct an extra PP, but PP deduction infrastructure
    // may not be fully implemented yet. Returning Number(1) for when it's supported.
    AbilityHandlerResult::Number(1)
}

