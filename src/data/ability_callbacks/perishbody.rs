//! Perish Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	perishbody: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
//! 			this.add('-ability', target, 'Perish Body');
//! 			source.addVolatile('perishsong');
//! 			target.addVolatile('perishsong');
//! 		},
//! 		flags: {},
//! 		name: "Perish Body",
//! 		rating: 1,
//! 		num: 253,
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

