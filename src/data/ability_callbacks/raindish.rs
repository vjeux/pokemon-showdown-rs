//! Rain Dish Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	raindish: {
//! 		onWeather(target, source, effect) {
//! 			if (target.hasItem('utilityumbrella')) return;
//! 			if (effect.id === 'raindance' || effect.id === 'primordialsea') {
//! 				this.heal(target.baseMaxhp / 16);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rain Dish",
//! 		rating: 1.5,
//! 		num: 44,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onWeather(target, source, effect)
pub fn on_weather(battle: &mut Battle, target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);

    // if (target.hasItem('utilityumbrella')) return;
    if target.has_item(&["utilityumbrella"]) {
        return AbilityHandlerResult::Undefined;
    }

    // if (effect.id === 'raindance' || effect.id === 'primordialsea')
    if effect.id == "raindance" || effect.id == "primordialsea" {
        // this.heal(target.baseMaxhp / 16);
        let heal_amount = target.base_maxhp / 16;
        battle.heal(heal_amount, target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
