//! Leaf Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	leafguard: {
//! 		onSetStatus(status, target, source, effect) {
//! 			if (['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
//! 				if ((effect as Move)?.status) {
//! 					this.add('-immune', target, '[from] ability: Leaf Guard');
//! 				}
//! 				return false;
//! 			}
//! 		},
//! 		onTryAddVolatile(status, target) {
//! 			if (status.id === 'yawn' && ['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
//! 				this.add('-immune', target, '[from] ability: Leaf Guard');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Leaf Guard",
//! 		rating: 0.5,
//! 		num: 102,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSetStatus(status, target, source, effect)
/// Prevents status conditions in sunny weather
pub fn on_set_status(battle: &mut Battle, _status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (['sunnyday', 'desolateland'].includes(target.effectiveWeather()))
    let eff_weather = target.effective_weather(&battle.field.get_weather().to_string());
    if eff_weather == "sunnyday" || eff_weather == "desolateland" {
        // if ((effect as Move)?.status)
        if effect.status.is_some() {
            // this.add('-immune', target, '[from] ability: Leaf Guard');
            battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Leaf Guard")]);
        }
        // return false;
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onTryAddVolatile(status, target)
/// Prevents yawn in sunny weather
pub fn on_try_add_volatile(battle: &mut Battle, status: &Status, target: &Pokemon) -> AbilityHandlerResult {
    // if (status.id === 'yawn' && ['sunnyday', 'desolateland'].includes(target.effectiveWeather()))
    if status.id == "yawn" {
        let eff_weather = target.effective_weather(&battle.field.get_weather().to_string());
        if eff_weather == "sunnyday" || eff_weather == "desolateland" {
            // this.add('-immune', target, '[from] ability: Leaf Guard');
            battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Leaf Guard")]);
            // return null;
            return AbilityHandlerResult::Null;
        }
    }
    AbilityHandlerResult::Undefined
}

