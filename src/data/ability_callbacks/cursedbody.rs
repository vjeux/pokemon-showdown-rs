//! Cursed Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cursedbody: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (source.volatiles['disable']) return;
//! 			if (!move.isMax && !move.flags['futuremove'] && move.id !== 'struggle') {
//! 				if (this.randomChance(3, 10)) {
//! 					source.addVolatile('disable', this.effectState.target);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Cursed Body",
//! 		rating: 2,
//! 		num: 130,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    /// 30% chance to disable the move that hit this Pokemon
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (source.volatiles['disable']) return;
        if source.has_volatile(&ID::new("disable")) {
            return AbilityHandlerResult::Undefined;
        }
        // if (!move.isMax && !move.flags['futuremove'] && move.id !== 'struggle')
        // Note: Checking for Max moves and Struggle only, skipping futuremove check
        if !move_.is_max && move_.id.as_str() != "struggle" {
            // if (this.randomChance(3, 10))
            if battle.random_chance(3, 10) {
                // source.addVolatile('disable', this.effectState.target);
                source.add_volatile(ID::new("disable"));
                battle.add("-start", &[
                    Arg::Pokemon(source),
                    Arg::Str("Disable"),
                    Arg::Str("[from] ability: Cursed Body"),
                    Arg::Str(&format!("[of] {}", target.name)),
                ]);
            }
        }
        AbilityHandlerResult::Undefined
    }
