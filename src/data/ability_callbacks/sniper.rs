//! Sniper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sniper: {
//! 		onModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).crit) {
//! 				this.debug('Sniper boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Sniper",
//! 		rating: 2,
//! 		num: 97,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyDamage(damage, source, target, move)
/// Boosts damage by 1.5x on critical hits
pub fn on_modify_damage(_battle: &mut Battle, _damage: u32, _source: &Pokemon, _target: &Pokemon, _move: &MoveDef, was_crit: bool) -> AbilityHandlerResult {
    // if (target.getMoveHitData(move).crit)
    if was_crit {
        // this.debug('Sniper boost');
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

