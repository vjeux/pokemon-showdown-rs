//! Turboblaze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	turboblaze: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Turboblaze');
//! 		},
//! 		onModifyMove(move) {
//! 			move.ignoreAbility = true;
//! 		},
//! 		flags: {},
//! 		name: "Turboblaze",
//! 		rating: 3,
//! 		num: 163,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Announces the ability when Pokemon enters battle
///
/// TODO: onStart handler needs Pokemon parameter
/// When implemented, should:
/// 1. Add message: this.add('-ability', pokemon, 'Turboblaze')
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes moves ignore target's abilities (same as Teravolt)
///
/// TODO: onModifyMove handler needs mutable MoveDef to set move.ignoreAbility
/// Currently onModifyMove takes &MoveDef (immutable reference)
/// When implemented, should:
/// 1. Set move.ignoreAbility = true
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // Requires mutable MoveDef to implement move.ignoreAbility = true
    // See teravolt.rs and battlebond.rs for similar issue
    AbilityHandlerResult::Undefined
}

