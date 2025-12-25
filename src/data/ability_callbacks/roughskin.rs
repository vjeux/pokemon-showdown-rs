//! Rough Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	roughskin: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target, true)) {
//! 				this.damage(source.baseMaxhp / 8, source, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rough Skin",
//! 		rating: 2.5,
//! 		num: 24,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit(damage, target, source, move)
/// Damages attacker by 1/8 of their max HP if they make contact
///
/// TODO: onDamagingHit handler not yet implemented
/// TODO: Needs checkMoveMakesContact(), source.baseMaxhp, battle.damage()
/// When implemented, should:
/// 1. Check if the move makes contact with target
/// 2. If contact made, damage the source for 1/8 of their baseMaxhp
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

