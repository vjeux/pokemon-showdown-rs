//! Flash Fire Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	flashfire: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Fire') {
//! 				move.accuracy = true;
//! 				if (!target.addVolatile('flashfire')) {
//! 					this.add('-immune', target, '[from] ability: Flash Fire');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onEnd(pokemon) {
//! 			pokemon.removeVolatile('flashfire');
//! 		},
//! 		condition: {
//! 			noCopy: true, // doesn't get copied by Baton Pass
//! 			onStart(target) {
//! 				this.add('-start', target, 'ability: Flash Fire');
//! 			},
//! 			onModifyAtkPriority: 5,
//! 			onModifyAtk(atk, attacker, defender, move) {
//! 				if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//! 					this.debug('Flash Fire boost');
//! 					return this.chainModify(1.5);
//! 				}
//! 			},
//! 			onModifySpAPriority: 5,
//! 			onModifySpA(atk, attacker, defender, move) {
//! 				if (move.type === 'Fire' && attacker.hasAbility('flashfire')) {
//! 					this.debug('Flash Fire boost');
//! 					return this.chainModify(1.5);
//! 				}
//! 			},
//! 			onEnd(target) {
//! 				this.add('-end', target, 'ability: Flash Fire', '[silent]');
//! 			},
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Flash Fire",
//! 		rating: 3.5,
//! 		num: 18,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
    pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        let target_ref = (target.side_index, target.position);
        let source_ref = (source.side_index, source.position);
        if target_ref != source_ref && move_.move_type == "Fire" {
            // Fire move hitting this pokemon - gain the boost
            if !target.add_volatile(ID::new("flashfire")) {
                battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Flash Fire")]);
            }
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        pokemon.remove_volatile(&ID::new("flashfire"));
        AbilityHandlerResult::Undefined
    }

    pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;
    pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

    /// onModifyAtk(atk, attacker, defender, move)
    pub fn on_modify_atk(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        if move_.move_type == "Fire" && attacker.has_ability(&["flashfire"]) && attacker.has_volatile(&ID::new("flashfire")) {
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(spa, attacker, defender, move)
    pub fn on_modify_spa(_spa: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        if move_.move_type == "Fire" && attacker.has_ability(&["flashfire"]) && attacker.has_volatile(&ID::new("flashfire")) {
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }

    // Condition handlers
    pub mod condition {

        
        use super::*;
/// onStart(target)
        pub fn on_start(battle: &mut Battle, target: &Pokemon) -> AbilityHandlerResult {
            battle.add("-start", &[Arg::Pokemon(target), Arg::Str("ability: Flash Fire")]);
            AbilityHandlerResult::Undefined
        }

        /// onEnd(target)
        pub fn on_end(battle: &mut Battle, target: &Pokemon) -> AbilityHandlerResult {
            battle.add("-end", &[Arg::Pokemon(target), Arg::Str("ability: Flash Fire"), Arg::Str("[silent]")]);
            AbilityHandlerResult::Undefined
        }
    }
