//! Rebound Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	rebound: {
//! 		isNonstandard: "CAP",
//! 		onTryHitPriority: 1,
//! 		onTryHit(target, source, move) {
//! 			if (this.effectState.target.activeTurns) return;
//! 
//! 			if (target === source || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
//! 				return;
//! 			}
//! 			const newMove = this.dex.getActiveMove(move.id);
//! 			newMove.hasBounced = true;
//! 			newMove.pranksterBoosted = false;
//! 			this.actions.useMove(newMove, target, { target: source });
//! 			return null;
//! 		},
//! 		onAllyTryHitSide(target, source, move) {
//! 			if (this.effectState.target.activeTurns) return;
//! 
//! 			if (target.isAlly(source) || move.hasBounced || !move.flags['reflectable'] || target.isSemiInvulnerable()) {
//! 				return;
//! 			}
//! 			const newMove = this.dex.getActiveMove(move.id);
//! 			newMove.hasBounced = true;
//! 			newMove.pranksterBoosted = false;
//! 			this.actions.useMove(newMove, this.effectState.target, { target: source });
//! 			move.hasBounced = true; // only bounce once in free-for-all battles
//! 			return null;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Rebound",
//! 		rating: 3,
//! 		num: -2,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHitPriority(...)
pub fn on_try_hit_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onTryHit(...)
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onAllyTryHitSide(...)
pub fn on_ally_try_hit_side(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

