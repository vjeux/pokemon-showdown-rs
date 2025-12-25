//! Flame Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	flamebody: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target)) {
//! 				if (this.randomChance(3, 10)) {
//! 					source.trySetStatus('brn', target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Flame Body",
//! 		rating: 2,
//! 		num: 49,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, _target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        let source_ref = (source.side_index, source.position);
        if battle.check_move_makes_contact(&move_.id, source_ref) {
            if battle.random_chance(3, 10) {
                source.try_set_status(ID::new("brn"), None);
            }
        }
        AbilityHandlerResult::Undefined
    }
