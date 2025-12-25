//! Disable Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	disable: {
//! 		num: 50,
//! 		accuracy: 100,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Disable",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { protect: 1, reflectable: 1, mirror: 1, bypasssub: 1, metronome: 1 },
//! 		volatileStatus: 'disable',
//! 		onTryHit(target) {
//! 			if (!target.lastMove || target.lastMove.isZOrMaxPowered || target.lastMove.isMax || target.lastMove.id === 'struggle') {
//! 				return false;
//! 			}
//! 		},
//! 		condition: {
//! 			duration: 5,
//! 			noCopy: true, // doesn't get copied by Baton Pass
//! 			onStart(pokemon, source, effect) {
//! 				// The target hasn't taken its turn, or Cursed Body activated and the move was not used through Dancer or Instruct
//! 				if (
//! 					this.queue.willMove(pokemon) ||
//! 					(pokemon === this.activePokemon && this.activeMove && !this.activeMove.isExternal)
//! 				) {
//! 					this.effectState.duration!--;
//! 				}
//! 				if (!pokemon.lastMove) {
//! 					this.debug(`Pokemon hasn't moved yet`);
//! 					return false;
//! 				}
//! 				for (const moveSlot of pokemon.moveSlots) {
//! 					if (moveSlot.id === pokemon.lastMove.id) {
//! 						if (!moveSlot.pp) {
//! 							this.debug('Move out of PP');
//! 							return false;
//! 						}
//! 					}
//! 				}
//! 				if (effect.effectType === 'Ability') {
//! 					this.add('-start', pokemon, 'Disable', pokemon.lastMove.name, '[from] ability: ' + effect.name, `[of] ${source}`);
//! 				} else {
//! 					this.add('-start', pokemon, 'Disable', pokemon.lastMove.name);
//! 				}
//! 				this.effectState.move = pokemon.lastMove.id;
//! 			},
//! 			onResidualOrder: 17,
//! 			onEnd(pokemon) {
//! 				this.add('-end', pokemon, 'Disable');
//! 			},
//! 			onBeforeMovePriority: 7,
//! 			onBeforeMove(attacker, defender, move) {
//! 				if (!(move.isZ && move.isZOrMaxPowered) && move.id === this.effectState.move) {
//! 					this.add('cant', attacker, 'Disable', move);
//! 					return false;
//! 				}
//! 			},
//! 			onDisableMove(pokemon) {
//! 				for (const moveSlot of pokemon.moveSlots) {
//! 					if (moveSlot.id === this.effectState.move) {
//! 						pokemon.disableMove(moveSlot.id);
//! 					}
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { effect: 'clearnegativeboost' },
//! 		contestType: "Clever",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit(...)
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
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

/// onEnd(...)
pub fn on_end(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
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

/// onDisableMove(...)
pub fn on_disable_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
