//! Stance Change Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	stancechange: {
//! 		onModifyMovePriority: 1,
//! 		onModifyMove(move, attacker, defender) {
//! 			if (attacker.species.baseSpecies !== 'Aegislash' || attacker.transformed) return;
//! 			if (move.category === 'Status' && move.id !== 'kingsshield') return;
//! 			const targetForme = (move.id === 'kingsshield' ? 'Aegislash' : 'Aegislash-Blade');
//! 			if (attacker.species.name !== targetForme) attacker.formeChange(targetForme);
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Stance Change",
//! 		rating: 4,
//! 		num: 176,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_MOVE_PRIORITY: i32 = 1;

/// onModifyMove(move, attacker, defender)
/// Aegislash forme changes based on move used
///
/// TODO: onModifyMove handler not yet implemented
/// TODO: Needs attacker.species.baseSpecies, attacker.transformed, move.category, move.id, attacker.species.name, attacker.formeChange()
/// When implemented, should:
/// 1. Skip if not Aegislash or if transformed
/// 2. Skip if move is Status category and not King's Shield
/// 3. Change to Shield Forme (Aegislash) if using King's Shield
/// 4. Change to Blade Forme (Aegislash-Blade) if using attacking move
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

