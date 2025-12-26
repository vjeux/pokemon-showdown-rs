//! Storm Drain Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	stormdrain: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Water') {
//! 				if (!this.boost({ spa: 1 })) {
//! 					this.add('-immune', target, '[from] ability: Storm Drain');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onAnyRedirectTarget(target, source, source2, move) {
//! 			if (move.type !== 'Water' || move.flags['pledgecombo']) return;
//! 			const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
//! 			if (this.validTarget(this.effectState.target, source, redirectTarget)) {
//! 				if (move.smartTarget) move.smartTarget = false;
//! 				if (this.effectState.target !== target) {
//! 					this.add('-activate', this.effectState.target, 'ability: Storm Drain');
//! 				}
//! 				return this.effectState.target;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Storm Drain",
//! 		rating: 3,
//! 		num: 114,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Grants immunity to Water-type moves and boosts Special Attack when hit
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && move.type === 'Water')
    if target_loc != source_loc && move_.move_type == "Water" {
        // if (!this.boost({ spa: 1 }))
        let boost_success = battle.boost(&[("spa", 1)], target_loc, Some(target_loc), None);

        if !boost_success {
            // this.add('-immune', target, '[from] ability: Storm Drain');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Storm Drain")
            ]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onAnyRedirectTarget(target, source, source2, move)
/// Redirects Water-type moves to this Pokemon
///
/// TODO: onAnyRedirectTarget handler not yet implemented
/// TODO: Needs move.type, move.flags, move.target, battle.validTarget(), battle.effectState.target
/// When implemented, should:
/// 1. Skip if move.type is not 'Water' or move has 'pledgecombo' flag
/// 2. Determine redirectTarget based on move.target ('randomNormal'/'adjacentFoe' -> 'normal', else use move.target)
/// 3. Check if effectState.target is valid target using validTarget()
/// 4. If valid, disable move.smartTarget if it exists
/// 5. Add activate message if redirecting to different target
/// 6. Return effectState.target to redirect the move
pub fn on_any_redirect_target(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

