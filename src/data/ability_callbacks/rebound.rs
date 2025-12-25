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

pub const ON_TRY_HIT_PRIORITY: i32 = 1;

/// onTryHit(target, source, move)
/// Bounces reflectable moves back (CAP ability, only works first turn)
///
/// TODO: onTryHit handler not yet implemented
/// TODO: Needs effectState.target.activeTurns, move.hasBounced, move.flags['reflectable'], target.isSemiInvulnerable(), dex.getActiveMove(), actions.useMove()
/// When implemented, should:
/// 1. Skip if not the Pokemon's first turn active
/// 2. Skip if target is source, move bounced, no reflectable flag, or target semi-invulnerable
/// 3. Create new move copy with hasBounced = true, pranksterBoosted = false
/// 4. Use the move from target back at source
/// 5. Return null to block the original move
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onAllyTryHitSide(target, source, move)
/// Bounces reflectable moves back for allies (CAP ability, only works first turn)
///
/// TODO: onAllyTryHitSide handler not yet implemented
/// TODO: Needs effectState.target.activeTurns, target.isAlly(), move.hasBounced, move.flags['reflectable'], target.isSemiInvulnerable()
/// When implemented, should:
/// 1. Skip if not the Pokemon's first turn active
/// 2. Skip if target is ally of source, move bounced, no reflectable flag, or target semi-invulnerable
/// 3. Create new move copy, use from ability holder back at source
/// 4. Set move.hasBounced = true to prevent multiple bounces in FFA
/// 5. Return null to block the original move
pub fn on_ally_try_hit_side(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

