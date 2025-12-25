//! Anger Point Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	angerpoint: {
//! 		onHit(target, source, move) {
//! 			if (!target.hp) return;
//! 			if (move?.effectType === 'Move' && target.getMoveHitData(move).crit) {
//! 				this.boost({ atk: 12 }, target, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Anger Point",
//! 		rating: 1,
//! 		num: 83,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onHit(target, source, move)
    /// Note: crit info passed as parameter since getMoveHitData isn't directly accessible
    pub fn on_hit(battle: &mut Battle, target: &Pokemon, _source: &Pokemon, _move: &MoveDef, was_crit: bool) -> AbilityHandlerResult {
        // if (!target.hp) return;
        if target.hp == 0 {
            return AbilityHandlerResult::Undefined;
        }
        // if (move?.effectType === 'Move' && target.getMoveHitData(move).crit)
        // Note: effectType check is implicit since we only call this for moves
        if was_crit {
            // this.boost({ atk: 12 }, target, target);
            let target_ref = (target.side_index, target.position);
            battle.boost(&[("atk", 12)], target_ref, Some(target_ref), None);
        }
        AbilityHandlerResult::Undefined
    }
