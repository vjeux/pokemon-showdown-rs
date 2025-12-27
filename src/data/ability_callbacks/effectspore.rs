//! Effect Spore Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	effectspore: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target) && !source.status && source.runStatusImmunity('powder')) {
//! 				const r = this.random(100);
//! 				if (r < 11) {
//! 					source.setStatus('slp', target);
//! 				} else if (r < 21) {
//! 					source.setStatus('par', target);
//! 				} else if (r < 30) {
//! 					source.setStatus('psn', target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Effect Spore",
//! 		rating: 2,
//! 		num: 27,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, _target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        let source_ref = (source.side_index, source.position);
        if battle.check_move_makes_contact(&move_.id, source_ref) && source.status.is_empty() && source.run_status_immunity("powder") {
            let r = battle.random(100);
            if r < 11 {
                source.set_status(ID::new("slp"));
            } else if r < 21 {
                source.set_status(ID::new("par"));
            } else if r < 30 {
                source.set_status(ID::new("psn"));
            }
        }
        AbilityHandlerResult::Undefined
    }
