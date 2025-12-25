//! Grim Neigh Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	grimneigh: {
//! 		onSourceAfterFaint(length, target, source, effect) {
//! 			if (effect && effect.effectType === 'Move') {
//! 				this.boost({ spa: length }, source);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Grim Neigh",
//! 		rating: 3,
//! 		num: 265,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceAfterFaint(length, target, source, effect)
pub fn on_source_after_faint(battle: &mut Battle, length: i32, _target: &Pokemon, source: &Pokemon, effect: &Effect) -> AbilityHandlerResult {
    let source_ref = (source.side_index, source.position);
    // if (effect && effect.effectType === 'Move')
    if effect.effect_type == "Move" {
        // this.boost({ spa: length }, source);
        battle.boost(&[("spa", length as i8)], source_ref, Some(source_ref), None);
    }
    AbilityHandlerResult::Undefined
}
