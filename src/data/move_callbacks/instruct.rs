//! Instruct Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	instruct: {
//! 		num: 689,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Instruct",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { protect: 1, bypasssub: 1, allyanim: 1, failinstruct: 1 },
//! 		onHit(target, source) {
//! 			if (!target.lastMove || target.volatiles['dynamax']) return false;
//! 			const lastMove = target.lastMove;
//! 			const moveSlot = target.getMoveData(lastMove.id);
//! 			if (
//! 				lastMove.flags['failinstruct'] || lastMove.isZ || lastMove.isMax ||
//! 				lastMove.flags['charge'] || lastMove.flags['recharge'] ||
//! 				target.volatiles['beakblast'] || target.volatiles['focuspunch'] || target.volatiles['shelltrap'] ||
//! 				(moveSlot && moveSlot.pp <= 0)
//! 			) {
//! 				return false;
//! 			}
//! 			this.add('-singleturn', target, 'move: Instruct', `[of] ${source}`);
//! 			this.queue.prioritizeAction(this.queue.resolveAction({
//! 				choice: 'move',
//! 				pokemon: target,
//! 				moveid: target.lastMove.id,
//! 				targetLoc: target.lastMoveTargetLoc!,
//! 			})[0] as MoveAction);
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Psychic",
//! 		zMove: { boost: { spa: 1 } },
//! 		contestType: "Clever",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

