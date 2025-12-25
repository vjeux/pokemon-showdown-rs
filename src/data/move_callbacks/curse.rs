//! Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	curse: {
//! 		num: 174,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Curse",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { bypasssub: 1, metronome: 1 },
//! 		volatileStatus: 'curse',
//! 		onModifyMove(move, source, target) {
//! 			if (!source.hasType('Ghost')) {
//! 				move.target = move.nonGhostTarget!;
//! 			} else if (source.isAlly(target)) {
//! 				move.target = 'randomNormal';
//! 			}
//! 		},
//! 		onTryHit(target, source, move) {
//! 			if (!source.hasType('Ghost')) {
//! 				delete move.volatileStatus;
//! 				delete move.onHit;
//! 				move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
//! 			} else if (move.volatileStatus && target.volatiles['curse']) {
//! 				return false;
//! 			}
//! 		},
//! 		onHit(target, source) {
//! 			this.directDamage(source.maxhp / 2, source, source);
//! 		},
//! 		condition: {
//! 			onStart(pokemon, source) {
//! 				this.add('-start', pokemon, 'Curse', `[of] ${source}`);
//! 			},
//! 			onResidualOrder: 12,
//! 			onResidual(pokemon) {
//! 				this.damage(pokemon.baseMaxhp / 4);
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		nonGhostTarget: "self",
//! 		type: "Ghost",
//! 		zMove: { effect: 'curse' },
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyMove(...)
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onTryHit(...)
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onResidualOrder(...)
pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onResidual(...)
pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
