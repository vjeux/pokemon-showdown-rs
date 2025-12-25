//! Rattled Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	rattled: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (['Dark', 'Bug', 'Ghost'].includes(move.type)) {
//! 				this.boost({ spe: 1 });
//! 			}
//! 		},
//! 		onAfterBoost(boost, target, source, effect) {
//! 			if (effect?.name === 'Intimidate' && boost.atk) {
//! 				this.boost({ spe: 1 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rattled",
//! 		rating: 1,
//! 		num: 155,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Boosts Speed when hit by Dark, Bug, or Ghost-type moves
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (['Dark', 'Bug', 'Ghost'].includes(move.type))
    let move_type = &move_.move_type;
    if move_type == "Dark" || move_type == "Bug" || move_type == "Ghost" {
        // this.boost({ spe: 1 });
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("spe", 1)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}

/// onAfterBoost(boost, target, source, effect)
/// Boosts Speed when affected by Intimidate
///
/// TODO: onAfterBoost handler not yet implemented
/// TODO: Needs effect.name, boost.atk, boost()
/// When implemented, should:
/// 1. If effect is Intimidate and boost.atk exists
/// 2. Boost Speed by 1 stage
pub fn on_after_boost(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

