//! Mountaineer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mountaineer: {
//! 		onDamage(damage, target, source, effect) {
//! 			if (effect && effect.id === 'stealthrock') {
//! 				return false;
//! 			}
//! 		},
//! 		onTryHit(target, source, move) {
//! 			if (move.type === 'Rock' && !target.activeTurns) {
//! 				this.add('-immune', target, '[from] ability: Mountaineer');
//! 				return null;
//! 			}
//! 		},
//! 		isNonstandard: "CAP",
//! 		flags: { breakable: 1 },
//! 		name: "Mountaineer",
//! 		rating: 3,
//! 		num: -1,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamage(damage, target, source, effect)
/// Prevents Stealth Rock damage
pub fn on_damage(_battle: &mut Battle, _damage: i32, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (effect && effect.id === 'stealthrock')
    if effect.id == "stealthrock" {
        // return false;
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onTryHit(target, source, move)
/// Grants immunity to Rock moves on switch-in
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Rock' && !target.activeTurns)
    if move_.move_type == "Rock" && target.active_turns == 0 {
        // this.add('-immune', target, '[from] ability: Mountaineer');
        battle.add("-immune", &[
            Arg::Pokemon(target),
            Arg::Str("[from] ability: Mountaineer")
        ]);
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

