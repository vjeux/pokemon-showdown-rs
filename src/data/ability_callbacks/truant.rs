//! Truant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	truant: {
//! 		onStart(pokemon) {
//! 			pokemon.removeVolatile('truant');
//! 			if (pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon))) {
//! 				pokemon.addVolatile('truant');
//! 			}
//! 		},
//! 		onBeforeMovePriority: 9,
//! 		onBeforeMove(pokemon) {
//! 			if (pokemon.removeVolatile('truant')) {
//! 				this.add('cant', pokemon, 'ability: Truant');
//! 				return false;
//! 			}
//! 			pokemon.addVolatile('truant');
//! 		},
//! 		condition: {},
//! 		flags: {},
//! 		name: "Truant",
//! 		rating: -1,
//! 		num: 54,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onBeforeMovePriority: 9
pub const ON_BEFORE_MOVE_PRIORITY: i32 = 9;

/// onStart(pokemon)
/// onBeforeMove(pokemon)
/// Pokemon can only move every other turn (loafing around)
///
/// TODO: Volatile status system not yet implemented
/// TODO: onBeforeMove handler not yet implemented
/// When implemented, should:
/// onStart:
/// 1. pokemon.removeVolatile('truant')
/// 2. Check if pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon))
/// 3. If true, pokemon.addVolatile('truant')
/// onBeforeMove:
/// 1. If pokemon.removeVolatile('truant') returns true:
///    - this.add('cant', pokemon, 'ability: Truant')
///    - return false
/// 2. Otherwise pokemon.addVolatile('truant')
pub fn on_start(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // Requires volatile status system, queue.willMove
    AbilityHandlerResult::Undefined
}

pub fn on_before_move(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // Requires volatile status system, onBeforeMove handler
    AbilityHandlerResult::Undefined
}

