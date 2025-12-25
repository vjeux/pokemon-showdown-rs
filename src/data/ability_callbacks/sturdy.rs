//! Sturdy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sturdy: {
//! 		onTryHit(pokemon, target, move) {
//! 			if (move.ohko) {
//! 				this.add('-immune', pokemon, '[from] ability: Sturdy');
//! 				return null;
//! 			}
//! 		},
//! 		onDamagePriority: -30,
//! 		onDamage(damage, target, source, effect) {
//! 			if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move') {
//! 				this.add('-ability', target, 'Sturdy');
//! 				return target.hp - 1;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Sturdy",
//! 		rating: 3,
//! 		num: 5,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(pokemon, target, move)
pub fn on_try_hit(battle: &mut Battle, pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.ohko)
    // Note: MoveDef doesn't have ohko field currently, skipping this check
    // This will need to be added to MoveDef later
    AbilityHandlerResult::Undefined
}

/// onDamagePriority: -30
pub const ON_DAMAGE_PRIORITY: i32 = -30;

/// onDamage(damage, target, source, effect)
pub fn on_damage(battle: &mut Battle, damage: u32, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move')
    if target.hp == target.maxhp && damage >= target.hp && effect.effect_type == "Move" {
        // this.add('-ability', target, 'Sturdy');
        battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Sturdy")]);
        // return target.hp - 1;
        return AbilityHandlerResult::Number((target.hp - 1) as i32);
    }
    AbilityHandlerResult::Undefined
}
