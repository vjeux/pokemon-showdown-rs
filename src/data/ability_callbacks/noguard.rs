//! No Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	noguard: {
//! 		onAnyInvulnerabilityPriority: 1,
//! 		onAnyInvulnerability(target, source, move) {
//! 			if (move && (source === this.effectState.target || target === this.effectState.target)) return 0;
//! 		},
//! 		onAnyAccuracy(accuracy, target, source, move) {
//! 			if (move && (source === this.effectState.target || target === this.effectState.target)) {
//! 				return true;
//! 			}
//! 			return accuracy;
//! 		},
//! 		flags: {},
//! 		name: "No Guard",
//! 		rating: 4,
//! 		num: 99,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_ANY_INVULNERABILITY_PRIORITY: i32 = 1;

/// onAnyInvulnerability(target, source, move)
/// Ensures moves bypass invulnerability (Fly, Dig, etc.)
///
/// TODO: onAnyInvulnerability handler not yet called by battle engine
pub fn on_any_invulnerability(_battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // if (move && (source === this.effectState.target || target === this.effectState.target)) return 0;
    let holder_ref = (ability_holder.side_index, ability_holder.position);
    let source_ref = (source.side_index, source.position);
    let target_ref = (target.side_index, target.position);

    if source_ref == holder_ref || target_ref == holder_ref {
        return AbilityHandlerResult::Number(0);
    }

    AbilityHandlerResult::Undefined
}

/// onAnyAccuracy(accuracy, target, source, move)
/// Ensures all moves involving this pokemon always hit
///
/// TODO: onAnyAccuracy handler not yet called by battle engine
pub fn on_any_accuracy(_battle: &mut Battle, accuracy: u32, target: &Pokemon, source: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // if (move && (source === this.effectState.target || target === this.effectState.target))
    let holder_ref = (ability_holder.side_index, ability_holder.position);
    let source_ref = (source.side_index, source.position);
    let target_ref = (target.side_index, target.position);

    if source_ref == holder_ref || target_ref == holder_ref {
        // return true;
        return AbilityHandlerResult::True;
    }

    // return accuracy;
    AbilityHandlerResult::Number(accuracy as i32)
}

