//! Mummy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mummy: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			const sourceAbility = source.getAbility();
//! 			if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'mummy') {
//! 				return;
//! 			}
//! 			if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
//! 				const oldAbility = source.setAbility('mummy', target);
//! 				if (oldAbility) {
//! 					this.add('-activate', target, 'ability: Mummy', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Mummy",
//! 		rating: 2,
//! 		num: 152,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Spreads Mummy to attackers on contact
///
/// TODO: onDamagingHit handler not yet implemented
/// TODO: Needs source.getAbility(), source.setAbility()
/// TODO: Needs ability.flags['cantsuppress'], isAlly() check
/// When implemented, should:
/// 1. Get source's ability and check if it has cantsuppress flag or is mummy
/// 2. Check if move makes contact using checkMoveMakesContact
/// 3. Set source's ability to 'mummy' and get old ability
/// 4. Add activate message with old ability name
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

