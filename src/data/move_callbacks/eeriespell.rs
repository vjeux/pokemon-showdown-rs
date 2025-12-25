//! Eerie Spell Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	eeriespell: {
//! 		num: 826,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Special",
//! 		name: "Eerie Spell",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1, sound: 1, bypasssub: 1, metronome: 1 },
//! 		secondary: {
//! 			chance: 100,
//! 			onHit(target) {
//! 				if (!target.hp) return;
//! 				let move: Move | ActiveMove | null = target.lastMove;
//! 				if (!move || move.isZ) return;
//! 				if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
//! 
//! 				const ppDeducted = target.deductPP(move.id, 3);
//! 				if (!ppDeducted) return;
//! 				this.add('-activate', target, 'move: Eerie Spell', move.name, ppDeducted);
//! 			},
//! 		},
//! 		target: "normal",
//! 		type: "Psychic",
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

