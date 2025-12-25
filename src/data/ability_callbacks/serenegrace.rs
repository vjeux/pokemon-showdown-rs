//! Serene Grace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	serenegrace: {
//! 		onModifyMovePriority: -2,
//! 		onModifyMove(move) {
//! 			if (move.secondaries) {
//! 				this.debug('doubling secondary chance');
//! 				for (const secondary of move.secondaries) {
//! 					if (secondary.chance) secondary.chance *= 2;
//! 				}
//! 			}
//! 			if (move.self?.chance) move.self.chance *= 2;
//! 		},
//! 		flags: {},
//! 		name: "Serene Grace",
//! 		rating: 3.5,
//! 		num: 32,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_MOVE_PRIORITY: i32 = -2;

/// onModifyMove(move)
/// Doubles the chance of secondary effects
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (move.secondaries)
    if !move_.secondaries.is_empty() {
        // this.debug('doubling secondary chance');
        // for (const secondary of move.secondaries)
        for secondary in &mut move_.secondaries {
            // if (secondary.chance) secondary.chance *= 2;
            if secondary.chance > 0 {
                secondary.chance = secondary.chance.saturating_mul(2).min(100);
            }
        }
    }

    // if (move.self?.chance) move.self.chance *= 2;
    // Note: move.self is not implemented in MoveDef yet
    // This would require a self_effect field similar to secondaries

    AbilityHandlerResult::Undefined
}

