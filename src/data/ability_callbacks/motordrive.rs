//! Motor Drive Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	motordrive: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Electric') {
//! 				if (!this.boost({ spe: 1 })) {
//! 					this.add('-immune', target, '[from] ability: Motor Drive');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Motor Drive",
//! 		rating: 3,
//! 		num: 78,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Absorbs Electric moves and boosts Speed by 1
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && move.type === 'Electric')
    if target_loc != source_loc && move_.move_type == "Electric" {
        // if (!this.boost({ spe: 1 }))
        let boost_success = battle.boost(&[("spe", 1)], target_loc, Some(target_loc), None);

        if !boost_success {
            // this.add('-immune', target, '[from] ability: Motor Drive');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Motor Drive")
            ]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }

    AbilityHandlerResult::Undefined
}

