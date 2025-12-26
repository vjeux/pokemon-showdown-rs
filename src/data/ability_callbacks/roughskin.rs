//! Rough Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	roughskin: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target, true)) {
//! 				this.damage(source.baseMaxhp / 8, source, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rough Skin",
//! 		rating: 2.5,
//! 		num: 24,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit(damage, target, source, move)
/// Damages attacker by 1/8 of their max HP if they make contact
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, _target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.checkMoveMakesContact(move, source, target, true))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // this.damage(source.baseMaxhp / 8, source, target);
        let damage_amount = source.base_maxhp / 8;
        battle.damage(damage_amount as i32, Some(source_ref), Some(source_ref), None, false);
    }
    AbilityHandlerResult::Undefined
}

