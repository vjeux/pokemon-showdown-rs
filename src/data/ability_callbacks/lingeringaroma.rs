//! Lingering Aroma Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	lingeringaroma: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			const sourceAbility = source.getAbility();
//! 			if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'lingeringaroma') {
//! 				return;
//! 			}
//! 			if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
//! 				const oldAbility = source.setAbility('lingeringaroma', target);
//! 				if (oldAbility) {
//! 					this.add('-activate', target, 'ability: Lingering Aroma', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Lingering Aroma",
//! 		rating: 2,
//! 		num: 268,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(...)
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

