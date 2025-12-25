//! Quick Draw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	quickdraw: {
//! 		onFractionalPriorityPriority: -1,
//! 		onFractionalPriority(priority, pokemon, target, move) {
//! 			if (move.category !== "Status" && this.randomChance(3, 10)) {
//! 				this.add('-activate', pokemon, 'ability: Quick Draw');
//! 				return 0.1;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Quick Draw",
//! 		rating: 2.5,
//! 		num: 259,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_FRACTIONAL_PRIORITY_PRIORITY: i32 = -1;

/// onFractionalPriority(priority, pokemon, target, move)
/// 30% chance to move first in its priority bracket
pub fn on_fractional_priority(battle: &mut Battle, _priority: f64, pokemon: &Pokemon, _target: Option<&Pokemon>, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category !== "Status" && this.randomChance(3, 10))
    if move_.category != MoveCategory::Status && battle.random_chance(3, 10) {
        // this.add('-activate', pokemon, 'ability: Quick Draw');
        battle.add("-activate", &[
            Arg::Pokemon(pokemon),
            Arg::Str("ability: Quick Draw")
        ]);
        // return 0.1;
        return AbilityHandlerResult::FractionalPriority(0.1);
    }
    AbilityHandlerResult::Undefined
}

