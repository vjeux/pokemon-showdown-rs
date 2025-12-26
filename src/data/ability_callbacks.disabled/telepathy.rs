//! Telepathy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	telepathy: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && target.isAlly(source) && move.category !== 'Status') {
//! 				this.add('-activate', target, 'ability: Telepathy');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Telepathy",
//! 		rating: 0,
//! 		num: 140,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
/// Prevents damage from ally attacks
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_loc = (target.side_index, target.position);
    let source_loc = (source.side_index, source.position);

    // if (target !== source && target.isAlly(source) && move.category !== 'Status')
    if target_loc != source_loc
        && target.is_ally(source.side_index)
        && move_.category != MoveCategory::Status
    {
        // this.add('-activate', target, 'ability: Telepathy');
        battle.add("-activate", &[
            Arg::Pokemon(target),
            Arg::Str("ability: Telepathy")
        ]);

        // return null;
        return AbilityHandlerResult::Null;
    }

    AbilityHandlerResult::Undefined
}

