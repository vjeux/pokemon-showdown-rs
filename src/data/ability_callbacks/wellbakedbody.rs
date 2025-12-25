//! Well-Baked Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	wellbakedbody: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Fire') {
//! 				if (!this.boost({ def: 2 })) {
//! 					this.add('-immune', target, '[from] ability: Well-Baked Body');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Well-Baked Body",
//! 		rating: 3.5,
//! 		num: 273,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Immune to Fire moves, raises Defense by 2 stages when hit
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && move.type === 'Fire')
    if target_loc != source_loc && move_.move_type == "Fire" {
        // if (!this.boost({ def: 2 }))
        let boost_success = battle.boost(&[("def", 2)], target_loc, Some(target_loc), None);

        if !boost_success {
            // this.add('-immune', target, '[from] ability: Well-Baked Body');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Well-Baked Body")
            ]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

