//! Mold Breaker Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	moldbreaker: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Mold Breaker');
//! 		},
//! 		onModifyMove(move) {
//! 			move.ignoreAbility = true;
//! 		},
//! 		flags: {},
//! 		name: "Mold Breaker",
//! 		rating: 3,
//! 		num: 104,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Shows message on switch-in
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-ability', pokemon, 'Mold Breaker');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Mold Breaker")]);
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes moves ignore target abilities
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // move.ignoreAbility = true;
    move_.ignores_ability = true;
    AbilityHandlerResult::Undefined
}

