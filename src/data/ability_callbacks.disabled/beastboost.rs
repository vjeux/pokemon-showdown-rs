//! Beast Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	beastboost: {
//! 		onSourceAfterFaint(length, target, source, effect) {
//! 			if (effect && effect.effectType === 'Move') {
//! 				const bestStat = source.getBestStat(true, true);
//! 				this.boost({ [bestStat]: length }, source);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Beast Boost",
//! 		rating: 3.5,
//! 		num: 224,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceAfterFaint(length, target, source, effect)
pub fn on_source_after_faint(battle: &mut Battle, length: i32, _target: &Pokemon, source: &Pokemon, effect: &Effect) -> AbilityHandlerResult {
    // if (effect && effect.effectType === 'Move')
    if effect.effect_type != "Move" {
        return AbilityHandlerResult::Undefined;
    }

    // const bestStat = source.getBestStat(true, true);
    let best_stat = source.get_best_stat(true);

    // this.boost({ [bestStat]: length }, source);
    let stat_str = best_stat.to_str();
    battle.boost(&[(stat_str, length as i8)], (source.side_index, source.position), Some((source.side_index, source.position)), None);

    AbilityHandlerResult::Undefined
}

