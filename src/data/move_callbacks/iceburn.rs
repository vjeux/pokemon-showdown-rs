//! Ice Burn Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	iceburn: {
//! 		num: 554,
//! 		accuracy: 90,
//! 		basePower: 140,
//! 		category: "Special",
//! 		name: "Ice Burn",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { charge: 1, protect: 1, mirror: 1, nosleeptalk: 1, failinstruct: 1 },
//! 		onTryMove(attacker, defender, move) {
//! 			if (attacker.removeVolatile(move.id)) {
//! 				return;
//! 			}
//! 			this.add('-prepare', attacker, move.name);
//! 			if (!this.runEvent('ChargeMove', attacker, defender, move)) {
//! 				return;
//! 			}
//! 			attacker.addVolatile('twoturnmove', defender);
//! 			return null;
//! 		},
//! 		secondary: {
//! 			chance: 30,
//! 			status: 'brn',
//! 		},
//! 		target: "normal",
//! 		type: "Ice",
//! 		contestType: "Beautiful",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryMove(...)
pub fn on_try_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

