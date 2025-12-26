//! Teravolt Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	teravolt: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Teravolt');
//! 		},
//! 		onModifyMove(move) {
//! 			move.ignoreAbility = true;
//! 		},
//! 		flags: {},
//! 		name: "Teravolt",
//! 		rating: 3,
//! 		num: 164,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Announces the ability when Pokemon enters battle
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-ability', pokemon, 'Teravolt');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Teravolt")]);
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes moves ignore target's abilities
///
/// TODO: onModifyMove handler needs mutable MoveDef to set move.ignoreAbility
/// Currently onModifyMove takes &MoveDef (immutable reference)
/// When implemented, should:
/// 1. Set move.ignoreAbility = true
pub fn on_modify_move(_battle: &mut Battle, _move: &MoveDef, _attacker: &Pokemon) -> AbilityHandlerResult {
    // TODO: Requires mutable MoveDef to implement move.ignoreAbility = true
    // See battlebond.rs for similar issue with move modification
    AbilityHandlerResult::Undefined
}

