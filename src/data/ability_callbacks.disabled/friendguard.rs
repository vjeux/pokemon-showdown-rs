//! Friend Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	friendguard: {
//! 		onAnyModifyDamage(damage, source, target, move) {
//! 			if (target !== this.effectState.target && target.isAlly(this.effectState.target)) {
//! 				this.debug('Friend Guard weaken');
//! 				return this.chainModify(0.75);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Friend Guard",
//! 		rating: 0,
//! 		num: 132,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAnyModifyDamage(damage, source, target, move)
    /// Reduces damage to allies by 25%
    pub fn on_any_modify_damage(_battle: &Battle, _damage: u32, _source: &Pokemon, target: &Pokemon, _move_: &MoveDef, effect_holder: &Pokemon) -> AbilityHandlerResult {
        // If target is an ally of the effect holder (but not the holder itself)
        let holder_ref = (effect_holder.side_index, effect_holder.position);
        let target_ref = (target.side_index, target.position);
        if target_ref != holder_ref && target.side_index == effect_holder.side_index {
            // 0.75x = 3072/4096
            return AbilityHandlerResult::ChainModify(3072, 4096);
        }
        AbilityHandlerResult::Undefined
    }
