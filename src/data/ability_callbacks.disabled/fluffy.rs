//! Fluffy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	fluffy: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			let mod = 1;
//! 			if (move.type === 'Fire') mod *= 2;
//! 			if (move.flags['contact']) mod /= 2;
//! 			return this.chainModify(mod);
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Fluffy",
//! 		rating: 3.5,
//! 		num: 218,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
/// Halves contact damage, doubles Fire damage
pub fn on_source_modify_damage(damage: u32, _source: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // let mod = 1;
    let mut mod_num = 4096;
    let mod_den = 4096;

    // if (move.type === 'Fire') mod *= 2;
    if move_.move_type == "Fire" {
        mod_num *= 2;
    }

    // if (move.flags['contact']) mod /= 2;
    if move_.flags.contact {
        mod_num /= 2;
    }

    // return this.chainModify(mod);
    if mod_num != mod_den {
        return AbilityHandlerResult::ChainModify(mod_num, mod_den);
    }
    AbilityHandlerResult::Undefined
}

