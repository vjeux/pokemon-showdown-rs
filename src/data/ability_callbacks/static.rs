//! Static Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	static: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target)) {
//! 				if (this.randomChance(3, 10)) {
//! 					source.trySetStatus('par', target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Static",
//! 		rating: 2,
//! 		num: 9,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, _target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.checkMoveMakesContact(move, source, target))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.trySetStatus('par', target);
            source.try_set_status(ID::new("par"), None);
        }
    }
    AbilityHandlerResult::Undefined
}

