//! Aftermath Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	aftermath: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (!target.hp && this.checkMoveMakesContact(move, source, target, true)) {
//! 				this.damage(source.baseMaxhp / 4, source, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Aftermath",
//! 		rating: 2,
//! 		num: 106,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// onDamagingHitOrder: 1,
    pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

    /// onDamagingHit(damage, target, source, move)
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (!target.hp && this.checkMoveMakesContact(move, source, target, true))
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        if target.hp == 0 && battle.check_move_makes_contact(&move_.id, source_ref) {
            // this.damage(source.baseMaxhp / 4, source, target);
            let damage = source.base_maxhp / 4;
            battle.damage(damage as i32, Some(source_ref), Some(target_ref), None, false);
        }
        AbilityHandlerResult::Undefined
    }
