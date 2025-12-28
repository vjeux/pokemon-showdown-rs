//! G-Max Depletion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					let move: Move | ActiveMove | null = pokemon.lastMove;
/// 					if (!move || move.isZ) continue;
/// 					if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
/// 
/// 					const ppDeducted = pokemon.deductPP(move.id, 2);
/// 					if (ppDeducted) {
/// 						this.add("-activate", pokemon, 'move: G-Max Depletion', move.name, ppDeducted);
/// 						// Don't return here because returning early doesn't trigger
/// 						// activation text for the second Pokemon in doubles
/// 					}
/// 				}
/// 			},
/// 
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

