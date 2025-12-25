//! Grudge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	grudge: {
//! 		num: 288,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Grudge",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { bypasssub: 1, metronome: 1 },
//! 		volatileStatus: 'grudge',
//! 		condition: {
//! 			onStart(pokemon) {
//! 				this.add('-singlemove', pokemon, 'Grudge');
//! 			},
//! 			onFaint(target, source, effect) {
//! 				if (!source || source.fainted || !effect) return;
//! 				if (effect.effectType === 'Move' && !effect.flags['futuremove'] && source.lastMove) {
//! 					let move: Move = source.lastMove;
//! 					if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
//! 
//! 					for (const moveSlot of source.moveSlots) {
//! 						if (moveSlot.id === move.id) {
//! 							moveSlot.pp = 0;
//! 							this.add('-activate', source, 'move: Grudge', move.name);
//! 						}
//! 					}
//! 				}
//! 			},
//! 			onBeforeMovePriority: 100,
//! 			onBeforeMove(pokemon) {
//! 				this.debug('removing Grudge before attack');
//! 				pokemon.removeVolatile('grudge');
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Ghost",
//! 		zMove: { effect: 'redirect' },
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onFaint(...)
pub fn on_faint(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onBeforeMovePriority(...)
pub fn on_before_move_priority(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onBeforeMove(...)
pub fn on_before_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
