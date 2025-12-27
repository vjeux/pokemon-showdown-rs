//! Poison Heal Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	poisonheal: {
//! 		onDamagePriority: 1,
//! 		onDamage(damage, target, source, effect) {
//! 			if (effect.id === 'psn' || effect.id === 'tox') {
//! 				this.heal(target.baseMaxhp / 8);
//! 				return false;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Poison Heal",
//! 		rating: 4,
//! 		num: 90,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagePriority: 1
pub const ON_DAMAGE_PRIORITY: i32 = 1;

/// onDamage(damage, target, source, effect)
pub fn on_damage(battle: &mut Battle, _damage: i32, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (effect.id === 'psn' || effect.id === 'tox')
    if effect.id == "psn" || effect.id == "tox" {
        // this.heal(target.baseMaxhp / 8);
        let target_ref = (target.side_index, target.position);
        let heal_amount = target.base_maxhp / 8;
        battle.heal(heal_amount as i32, Some(target_ref), Some(target_ref), None);
        // return false;
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}
