//! Poison Touch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	poisontouch: {
//! 		onSourceDamagingHit(damage, target, source, move) {
//! 			// Despite not being a secondary, Shield Dust / Covert Cloak block Poison Touch's effect
//! 			if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
//! 			if (this.checkMoveMakesContact(move, target, source)) {
//! 				if (this.randomChance(3, 10)) {
//! 					target.trySetStatus('psn', source);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Poison Touch",
//! 		rating: 2,
//! 		num: 143,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceDamagingHit(damage, target, source, move)
/// 30% chance to poison the target when hitting with a contact move
///
/// TODO: onSourceDamagingHit handler not yet called by battle engine
/// TODO: Item checking (hasItem) not yet available
/// TODO: Needs mutable target reference to call try_set_status
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: i32, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // Despite not being a secondary, Shield Dust / Covert Cloak block Poison Touch's effect
    // if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
    if target.ability.as_str() == "shielddust" {
        return AbilityHandlerResult::Undefined;
    }
    // TODO: target.hasItem('covertcloak') check not yet available

    // if (this.checkMoveMakesContact(move, target, source))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // target.trySetStatus('psn', source);
            // TODO: Would need mutable target to call try_set_status
            // For now, this is implemented as far as possible without that
        }
    }

    AbilityHandlerResult::Undefined
}

