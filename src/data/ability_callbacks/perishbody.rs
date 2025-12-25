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

/// onDamagingHit(damage, target, source, move)
/// Inflicts Perish Song on both self and attacker when hit by contact move
///
/// TODO: onDamagingHit handler not yet implemented
/// TODO: Needs checkMoveMakesContact(), source.volatiles, addVolatile()
/// When implemented, should:
/// 1. Skip if move doesn't make contact or source already has perishsong
/// 2. Add ability message
/// 3. Add perishsong volatile to both source and target
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

