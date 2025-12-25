//! Sap Sipper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sapsipper: {
//! 		onTryHitPriority: 1,
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Grass') {
//! 				if (!this.boost({ atk: 1 })) {
//! 					this.add('-immune', target, '[from] ability: Sap Sipper');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onAllyTryHitSide(target, source, move) {
//! 			if (source === this.effectState.target || !target.isAlly(source)) return;
//! 			if (move.type === 'Grass') {
//! 				this.boost({ atk: 1 }, this.effectState.target);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Sap Sipper",
//! 		rating: 3,
//! 		num: 157,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_TRY_HIT_PRIORITY: i32 = 1;

/// onTryHit(target, source, move)
/// Blocks Grass moves and boosts Attack instead
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && move.type === 'Grass')
    if target_loc != source_loc && move_.move_type == "Grass" {
        // if (!this.boost({ atk: 1 }))
        let boost_success = battle.boost(&[("atk", 1)], target_loc, Some(target_loc), None);

        if !boost_success {
            // this.add('-immune', target, '[from] ability: Sap Sipper');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Sap Sipper")
            ]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onAllyTryHitSide(target, source, move)
/// Boosts Attack when ally is targeted by Grass move
///
/// TODO: onAllyTryHitSide handler not yet implemented
/// TODO: Needs effectState.target, target.isAlly(), move.type, boost()
/// When implemented, should:
/// 1. Skip if source is the ability holder or target is not ally of source
/// 2. If move is Grass-type
/// 3. Boost the ability holder's Attack by 1 stage
pub fn on_ally_try_hit_side(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

