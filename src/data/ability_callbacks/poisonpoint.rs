//! Poison Point Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	poisonpoint: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target)) {
//! 				if (this.randomChance(3, 10)) {
//! 					source.trySetStatus('psn', target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Poison Point",
//! 		rating: 1.5,
//! 		num: 38,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// 30% chance to poison the attacker when hit by a contact move
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, _target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.checkMoveMakesContact(move, source, target))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.trySetStatus('psn', target);
            source.try_set_status(ID::new("psn"), None);
        }
    }
    AbilityHandlerResult::Undefined
}

