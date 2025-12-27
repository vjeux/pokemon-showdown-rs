//! Punk Rock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	punkrock: {
//! 		onBasePowerPriority: 7,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (move.flags['sound']) {
//! 				this.debug('Punk Rock boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (move.flags['sound']) {
//! 				this.debug('Punk Rock weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Punk Rock",
//! 		rating: 3.5,
//! 		num: 244,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_BASE_POWER_PRIORITY: i32 = 7;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts sound-based moves by 1.3x
pub fn on_base_power(_base_power: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['sound'])
    if move_.flags.sound {
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

/// onSourceModifyDamage(damage, source, target, move)
/// Halves damage from sound-based moves
pub fn on_source_modify_damage(_damage: i32, _source: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['sound'])
    if move_.flags.sound {
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

