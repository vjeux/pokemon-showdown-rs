//! Wonder Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	wonderguard: {
//! 		onTryHit(target, source, move) {
//! 			if (target === source || move.category === 'Status' || move.id === 'struggle') return;
//! 			if (move.id === 'skydrop' && !source.volatiles['skydrop']) return;
//! 			this.debug('Wonder Guard immunity: ' + move.id);
//! 			if (target.runEffectiveness(move) <= 0 || !target.runImmunity(move)) {
//! 				if (move.smartTarget) {
//! 					move.smartTarget = false;
//! 				} else {
//! 					this.add('-immune', target, '[from] ability: Wonder Guard');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, failskillswap: 1, breakable: 1 },
//! 		name: "Wonder Guard",
//! 		rating: 5,
//! 		num: 25,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::data::typechart::get_effectiveness_multi;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
pub fn on_try_hit(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (target === source || move.category === 'Status' || move.id === 'struggle') return;
    if (target.side_index == source.side_index && target.position == source.position) ||
       move_.category == MoveCategory::Status ||
       move_.id.as_str() == "struggle" {
        return AbilityHandlerResult::Undefined;
    }

    // if (move.id === 'skydrop' && !source.volatiles['skydrop']) return;
    if move_.id.as_str() == "skydrop" && !source.volatiles.contains_key(&ID::new("skydrop")) {
        return AbilityHandlerResult::Undefined;
    }

    // this.debug('Wonder Guard immunity: ' + move.id);
    // if (target.runEffectiveness(move) <= 0 || !target.runImmunity(move))
    let effectiveness = get_effectiveness_multi(&move_.move_type, &target.types);

    if effectiveness <= 1.0 {
        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-immune', target, '[from] ability: Wonder Guard');
        // }
        // Note: smartTarget handling would require mutable move
        battle.add("-immune", &[
            Arg::Pokemon(target),
            Arg::Str("[from] ability: Wonder Guard")
        ]);
        // return null;
        return AbilityHandlerResult::Null;
    }

    AbilityHandlerResult::Undefined
}

