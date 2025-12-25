//! Moxie Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	moxie: {
//! 		onSourceAfterFaint(length, target, source, effect) {
//! 			if (effect && effect.effectType === 'Move') {
//! 				this.boost({ atk: length }, source);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Moxie",
//! 		rating: 3,
//! 		num: 153,
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
        // this.boost({ atk: length }, source);
        battle.boost(&[("atk", length as i8)], source_ref, Some(source_ref), None);
    }
    AbilityHandlerResult::Undefined
}
