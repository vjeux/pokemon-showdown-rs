//! Pursuit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	pursuit: {
//! 		num: 228,
//! 		accuracy: 100,
//! 		basePower: 40,
//! 		basePowerCallback(pokemon, target, move) {
//! 			// You can't get here unless the pursuit succeeds
//! 			if (target.beingCalledBack || target.switchFlag) {
//! 				this.debug('Pursuit damage boost');
//! 				return move.basePower * 2;
//! 			}
//! 			return move.basePower;
//! 		},
//! 		category: "Physical",
//! 		isNonstandard: "Past",
//! 		name: "Pursuit",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1 },
//! 		beforeTurnCallback(pokemon) {
//! 			for (const side of this.sides) {
//! 				if (side.hasAlly(pokemon)) continue;
//! 				side.addSideCondition('pursuit', pokemon);
//! 				const data = side.getSideConditionData('pursuit');
//! 				if (!data.sources) {
//! 					data.sources = [];
//! 				}
//! 				data.sources.push(pokemon);
//! 			}
//! 		},
//! 		onModifyMove(move, source, target) {
//! 			if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
//! 		},
//! 		onTryHit(target, pokemon) {
//! 			target.side.removeSideCondition('pursuit');
//! 		},
//! 		condition: {
//! 			duration: 1,
//! 			onBeforeSwitchOut(pokemon) {
//! 				this.debug('Pursuit start');
//! 				let alreadyAdded = false;
//! 				pokemon.removeVolatile('destinybond');
//! 				for (const source of this.effectState.sources) {
//! 					if (!source.isAdjacent(pokemon) || !this.queue.cancelMove(source) || !source.hp) continue;
//! 					if (!alreadyAdded) {
//! 						this.add('-activate', pokemon, 'move: Pursuit');
//! 						alreadyAdded = true;
//! 					}
//! 					// Run through each action in queue to check if the Pursuit user is supposed to Mega Evolve this turn.
//! 					// If it is, then Mega Evolve before moving.
//! 					if (source.canMegaEvo || source.canUltraBurst || source.canTerastallize) {
//! 						for (const [actionIndex, action] of this.queue.entries()) {
//! 							if (action.pokemon === source) {
//! 								if (action.choice === 'megaEvo') {
//! 									this.actions.runMegaEvo(source);
//! 								} else if (action.choice === 'terastallize') {
//! 									// Also a "forme" change that happens before moves, though only possible in NatDex
//! 									this.actions.terastallize(source);
//! 								} else {
//! 									continue;
//! 								}
//! 								this.queue.list.splice(actionIndex, 1);
//! 								break;
//! 							}
//! 						}
//! 					}
//! 					this.actions.runMove('pursuit', source, source.getLocOf(pokemon));
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Dark",
//! 		contestType: "Clever",
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

/// onBeforeSwitchOut(...)
pub fn on_before_switch_out(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
