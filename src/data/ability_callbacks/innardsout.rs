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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHitOrder: 1
pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit(damage, target, source, move)
/// Deals damage equal to damage taken when KO'd
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target: &Pokemon, source: &mut Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // if (!target.hp)
    if target.hp == 0 {
        // this.damage(target.getUndynamaxedHP(damage), source, target);
        // In JS, getUndynamaxedHP(damage) takes damage as param
        // In Rust, we just use the damage value directly since Dynamax isn't implemented
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        battle.damage(damage as i32, Some(source_ref), Some(target_ref), None, false);
    }
    AbilityHandlerResult::Undefined
}

