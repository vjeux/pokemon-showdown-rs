//! Orichalcum Pulse Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	orichalcumpulse: {
//! 		onStart(pokemon) {
//! 			if (this.field.setWeather('sunnyday')) {
//! 				this.add('-activate', pokemon, 'Orichalcum Pulse', '[source]');
//! 			} else if (this.field.isWeather('sunnyday')) {
//! 				this.add('-activate', pokemon, 'ability: Orichalcum Pulse');
//! 			}
//! 		},
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, pokemon) {
//! 			if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
//! 				this.debug('Orichalcum boost');
//! 				return this.chainModify([5461, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Orichalcum Pulse",
//! 		rating: 4.5,
//! 		num: 288,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onStart(pokemon)
/// Sets Sunny Day on switch-in
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.field.setWeather('sunnyday'))
    let weather_changed = battle.field.set_weather(ID::new("sunnyday"), None);
    if weather_changed {
        // this.add('-activate', pokemon, 'Orichalcum Pulse', '[source]');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("Orichalcum Pulse"), Arg::Str("[source]")]);
    } else if battle.field.is_weather("sunnyday") {
        // else if (this.field.isWeather('sunnyday'))
        // this.add('-activate', pokemon, 'ability: Orichalcum Pulse');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Orichalcum Pulse")]);
    }
    AbilityHandlerResult::Undefined
}

/// onModifyAtk(atk, pokemon)
/// Boosts Attack by 1.333x in sun
pub fn on_modify_atk(battle: &mut Battle, _atk: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
    let weather = battle.field.effective_weather();
    if *weather == ID::new("sunnyday") || *weather == ID::new("desolateland") {
        // this.debug('Orichalcum boost');
        // return this.chainModify([5461, 4096]);
        return AbilityHandlerResult::ChainModify(5461, 4096); // ~1.333x
    }
    AbilityHandlerResult::Undefined
}

