//! Overcoat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	overcoat: {
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
//! 		},
//! 		onTryHitPriority: 1,
//! 		onTryHit(target, source, move) {
//! 			if (move.flags['powder'] && target !== source && this.dex.getImmunity('powder', target)) {
//! 				this.add('-immune', target, '[from] ability: Overcoat');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Overcoat",
//! 		rating: 2,
//! 		num: 142,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onImmunity(type, pokemon)
/// Grants immunity to sandstorm, hail, and powder
///
/// TODO: onImmunity handler not yet implemented
/// When implemented, should:
/// 1. Check if type is sandstorm, hail, or powder
/// 2. Return false to grant immunity
pub fn on_immunity(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

pub const ON_TRY_HIT_PRIORITY: i32 = 1;

/// onTryHit(target, source, move)
/// Blocks powder moves
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (move.flags['powder'] && target !== source && this.dex.getImmunity('powder', target))
    // TODO: Missing dex.getImmunity() check - currently assuming powder immunity is respected
    if move_.flags.powder && target_loc != source_loc {
        // this.add('-immune', target, '[from] ability: Overcoat');
        battle.add("-immune", &[
            Arg::Pokemon(target),
            Arg::Str("[from] ability: Overcoat")
        ]);
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

