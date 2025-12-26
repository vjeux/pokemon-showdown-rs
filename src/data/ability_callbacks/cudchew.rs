//! Cud Chew Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cudchew: {
//! 		onEatItem(item, pokemon, source, effect) {
//! 			if (item.isBerry && (!effect || !['bugbite', 'pluck'].includes(effect.id))) {
//! 				this.effectState.berry = item;
//! 				this.effectState.counter = 2;
//! 				// This is needed in case the berry was eaten during residuals, preventing the timer from decreasing this turn
//! 				if (!this.queue.peek()) this.effectState.counter--;
//! 			}
//! 		},
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			if (!this.effectState.berry || !pokemon.hp) return;
//! 			if (--this.effectState.counter <= 0) {
//! 				const item = this.effectState.berry;
//! 				this.add('-activate', pokemon, 'ability: Cud Chew');
//! 				this.add('-enditem', pokemon, item.name, '[eat]');
//! 				if (this.singleEvent('Eat', item, null, pokemon, null, null)) {
//! 					this.runEvent('EatItem', pokemon, null, null, item);
//! 				}
//! 				if (item.onEat) pokemon.ateBerry = true;
//! 				delete this.effectState.berry;
//! 				delete this.effectState.counter;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Cud Chew",
//! 		rating: 2,
//! 		num: 291,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onResidualOrder: 28
    pub const ON_RESIDUAL_ORDER: i32 = 28;
    /// onResidualSubOrder: 2
    pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

    /// onEatItem(item, pokemon, source, effect)
    /// Stores the berry to re-eat next turn
    pub fn on_eat_item(pokemon: &mut Pokemon, item_id: &str, is_berry: bool, effect_id: Option<&str>) -> AbilityHandlerResult {
        // if (item.isBerry && (!effect || !['bugbite', 'pluck'].includes(effect.id)))
        if is_berry {
            if let Some(eff) = effect_id {
                if eff == "bugbite" || eff == "pluck" {
                    return AbilityHandlerResult::Undefined;
                }
            }
            // Store berry in ability state
            pokemon.ability_state.data.insert("berry".to_string(), serde_json::Value::String(item_id.to_string()));
            pokemon.ability_state.data.insert("counter".to_string(), serde_json::Value::Number(2.into()));
        }
        AbilityHandlerResult::Undefined
    }

    /// onResidual(pokemon)
    /// Re-eat the stored berry after countdown
    pub fn on_residual(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        // if (!this.effectState.berry || !pokemon.hp) return;
        let berry = match pokemon.ability_state.data.get("berry") {
            Some(serde_json::Value::String(b)) => b.clone(),
            _ => return AbilityHandlerResult::Undefined,
        };
        if pokemon.hp == 0 {
            return AbilityHandlerResult::Undefined;
        }
        // if (--this.effectState.counter <= 0)
        let counter: i64 = pokemon.ability_state.data.get("counter")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) - 1;
        if counter <= 0 {
            // this.add('-activate', pokemon, 'ability: Cud Chew');
            battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Cud Chew")]);
            // this.add('-enditem', pokemon, item.name, '[eat]');
            battle.add("-enditem", &[Arg::Pokemon(pokemon), Arg::Str(&berry), Arg::Str("[eat]")]);
            // Mark berry as eaten
            pokemon.ate_berry = true;
            // Clean up state
            pokemon.ability_state.data.remove("berry");
            pokemon.ability_state.data.remove("counter");
        } else {
            pokemon.ability_state.data.insert("counter".to_string(), serde_json::Value::Number(counter.into()));
        }
        AbilityHandlerResult::Undefined
    }
