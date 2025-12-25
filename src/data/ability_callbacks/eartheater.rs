//! Earth Eater Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	eartheater: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Ground') {
//! 				if (!this.heal(target.baseMaxhp / 4)) {
//! 					this.add('-immune', target, '[from] ability: Earth Eater');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Earth Eater",
//! 		rating: 3.5,
//! 		num: 297,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
    pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        let target_ref = (target.side_index, target.position);
        if target_ref != (source.side_index, source.position) && move_.move_type == "Ground" {
            let heal_amount = target.maxhp / 4;
            if battle.heal(heal_amount, target_ref, None, None) == 0 {
                battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Earth Eater")]);
            }
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }
