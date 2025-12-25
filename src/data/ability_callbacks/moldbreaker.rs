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
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Shows message on switch-in
///
/// TODO: onStart handler not yet implemented
/// When implemented, should:
/// 1. Add '-ability' message for Mold Breaker
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // this.add('-ability', pokemon, 'Mold Breaker');
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes moves ignore target abilities
///
/// TODO: onModifyMove handler not yet implemented
/// TODO: Needs move.ignoreAbility field
/// When implemented, should:
/// 1. Set move.ignoreAbility = true
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // move.ignoreAbility = true;
    AbilityHandlerResult::Undefined
}

