//! Innards Out Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	innardsout: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (!target.hp) {
//! 				this.damage(target.getUndynamaxedHP(damage), source, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Innards Out",
//! 		rating: 4,
//! 		num: 215,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHitOrder: 1
pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit(damage, target, source, move)
/// Deals damage equal to damage taken when KO'd
///
/// TODO: Partially implementable - onDamagingHit exists but needs verification
/// TODO: JS calls target.getUndynamaxedHP(damage) which takes damage as parameter
/// TODO: Rust has pokemon.get_undynamaxed_hp() with no parameters
/// TODO: Need to verify if getUndynamaxedHP should take damage param or use damage directly
/// When implemented, should:
/// 1. Check if !target.hp (target fainted)
/// 2. Call this.damage(amount, source, target) where amount is undynamaxed damage
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // if (!target.hp) {
    //     this.damage(target.getUndynamaxedHP(damage), source, target);
    // }
    AbilityHandlerResult::Undefined
}

