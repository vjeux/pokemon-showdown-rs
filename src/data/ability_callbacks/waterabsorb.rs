//! Water Absorb Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	waterabsorb: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Water') {
//! 				if (!this.heal(target.baseMaxhp / 4)) {
//! 					this.add('-immune', target, '[from] ability: Water Absorb');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Water Absorb",
//! 		rating: 3.5,
//! 		num: 11,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
pub fn on_try_hit(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    let source_ref = (source.side_index, source.position);

    // if (target !== source && move.type === 'Water')
    if target_ref != source_ref && move_.move_type == "Water" {
        // if (!this.heal(target.baseMaxhp / 4))
        let heal_amount = target.base_maxhp / 4;
        let healed = battle.heal(heal_amount as i32, Some(target_ref), Some(target_ref), None);

        if healed == Some(0) {
            // this.add('-immune', target, '[from] ability: Water Absorb');
            battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Water Absorb")]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}
