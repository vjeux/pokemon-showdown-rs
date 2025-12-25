//! Ice Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	icebody: {
//! 		onWeather(target, source, effect) {
//! 			if (effect.id === 'hail' || effect.id === 'snowscape') {
//! 				this.heal(target.baseMaxhp / 16);
//! 			}
//! 		},
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'hail') return false;
//! 		},
//! 		flags: {},
//! 		name: "Ice Body",
//! 		rating: 1,
//! 		num: 115,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onWeather(target, source, effect)
/// Heals 1/16 HP in Hail or Snowscape weather
pub fn on_weather(battle: &mut Battle, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (effect.id === 'hail' || effect.id === 'snowscape')
    if effect.id == "hail" || effect.id == "snowscape" {
        // this.heal(target.baseMaxhp / 16);
        let heal_amount = target.base_maxhp / 16;
        let target_ref = (target.side_index, target.position);
        battle.heal(heal_amount, target_ref, None, None);
    }
    AbilityHandlerResult::Undefined
}

/// onImmunity(type, pokemon)
/// Grants immunity to hail damage
pub fn on_immunity(_battle: &mut Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'hail') return false;
    if immunity_type == "hail" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

