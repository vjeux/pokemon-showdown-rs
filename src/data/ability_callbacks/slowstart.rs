//! Slow Start Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	slowstart: {
//! 		onStart(pokemon) {
//! 			this.add('-start', pokemon, 'ability: Slow Start');
//! 			this.effectState.counter = 5;
//! 		},
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			if (pokemon.activeTurns && this.effectState.counter) {
//! 				this.effectState.counter--;
//! 				if (!this.effectState.counter) {
//! 					this.add('-end', pokemon, 'Slow Start');
//! 					delete this.effectState.counter;
//! 				}
//! 			}
//! 		},
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, pokemon) {
//! 			if (this.effectState.counter) {
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onModifySpe(spe, pokemon) {
//! 			if (this.effectState.counter) {
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Slow Start",
//! 		rating: -1,
//! 		num: 112,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // this.add('-start', pokemon, 'ability: Slow Start');
    battle.add("-start", &[Arg::Pokemon(pokemon), Arg::Str("ability: Slow Start")]);
    // this.effectState.counter = 5;
    pokemon.ability_state.data.insert("counter".to_string(), serde_json::Value::Number(5.into()));
    AbilityHandlerResult::Undefined
}

/// onResidualOrder: 28
pub const ON_RESIDUAL_ORDER: i32 = 28;
/// onResidualSubOrder: 2
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
pub fn on_residual(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.activeTurns && this.effectState.counter)
    if pokemon.active_turns > 0 {
        if let Some(counter_val) = pokemon.ability_state.data.get_mut("counter") {
            if let Some(counter) = counter_val.as_i64() {
                if counter > 0 {
                    // this.effectState.counter--;
                    *counter_val = serde_json::Value::Number((counter - 1).into());

                    // if (!this.effectState.counter)
                    if counter - 1 == 0 {
                        // this.add('-end', pokemon, 'Slow Start');
                        battle.add("-end", &[Arg::Pokemon(pokemon), Arg::Str("Slow Start")]);
                        // delete this.effectState.counter;
                        pokemon.ability_state.data.remove("counter");
                    }
                }
            }
        }
    }
    AbilityHandlerResult::Undefined
}

/// onModifyAtkPriority: 5
pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, pokemon)
pub fn on_modify_atk(_battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.effectState.counter)
    if let Some(counter) = pokemon.ability_state.data.get("counter").and_then(|v| v.as_i64()) {
        if counter > 0 {
            // return this.chainModify(0.5);
            return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
        }
    }
    AbilityHandlerResult::Undefined
}

/// onModifySpe(spe, pokemon)
pub fn on_modify_spe(_battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.effectState.counter)
    if let Some(counter) = pokemon.ability_state.data.get("counter").and_then(|v| v.as_i64()) {
        if counter > 0 {
            // return this.chainModify(0.5);
            return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
        }
    }
    AbilityHandlerResult::Undefined
}

