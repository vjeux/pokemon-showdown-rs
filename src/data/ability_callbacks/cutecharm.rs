//! Cute Charm Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cutecharm: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target)) {
//! 				if (this.randomChance(3, 10)) {
//! 					source.addVolatile('attract', this.effectState.target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Cute Charm",
//! 		rating: 0.5,
//! 		num: 56,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    /// 30% chance to infatuate an attacker of the opposite gender that makes contact
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (this.checkMoveMakesContact(move, source, target))
        let target_ref = (target.side_index, target.position);
        if battle.check_move_makes_contact(&move_.id, target_ref) {
            // if (this.randomChance(3, 10))
            if battle.random_chance(3, 10) {
                // source.addVolatile('attract', this.effectState.target);
                source.add_volatile(ID::new("attract"));
                battle.add("-start", &[
                    Arg::Pokemon(source),
                    Arg::Str("Attract"),
                    Arg::Str("[from] ability: Cute Charm"),
                    Arg::Str(&format!("[of] {}", target.name)),
                ]);
            }
        }
        AbilityHandlerResult::Undefined
    }
