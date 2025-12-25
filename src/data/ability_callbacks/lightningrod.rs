//! Lightning Rod Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	lightningrod: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Electric') {
//! 				if (!this.boost({ spa: 1 })) {
//! 					this.add('-immune', target, '[from] ability: Lightning Rod');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onAnyRedirectTarget(target, source, source2, move) {
//! 			if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
//! 			const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
//! 			if (this.validTarget(this.effectState.target, source, redirectTarget)) {
//! 				if (move.smartTarget) move.smartTarget = false;
//! 				if (this.effectState.target !== target) {
//! 					this.add('-activate', this.effectState.target, 'ability: Lightning Rod');
//! 				}
//! 				return this.effectState.target;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Lightning Rod",
//! 		rating: 3,
//! 		num: 31,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Absorbs Electric moves and boosts SpA
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && move.type === 'Electric')
    if target_loc != source_loc && move_.move_type == "Electric" {
        // if (!this.boost({ spa: 1 }))
        let boost_success = battle.boost(&[("spa", 1)], target_loc, Some(target_loc), None);

        if !boost_success {
            // this.add('-immune', target, '[from] ability: Lightning Rod');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Lightning Rod")
            ]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }

    AbilityHandlerResult::Undefined
}

/// onAnyRedirectTarget(target, source, source2, move)
/// Redirects Electric moves to this Pokemon in doubles
///
/// TODO: onAnyRedirectTarget handler not yet implemented
/// When implemented, should:
/// 1. Check if move.type === 'Electric' and not pledgecombo
/// 2. Determine redirect target based on move.target
/// 3. Validate target and redirect if valid
/// 4. Add activate message and return effectState.target
pub fn on_any_redirect_target(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

