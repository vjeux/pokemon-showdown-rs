//! Snow Warning Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	snowwarning: {
//! 		onStart(source) {
//! 			this.field.setWeather('snowscape');
//! 		},
//! 		flags: {},
//! 		name: "Snow Warning",
//! 		rating: 4,
//! 		num: 117,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setWeather('snowscape');
    battle.field.set_weather(ID::new("snowscape"), None);
    AbilityHandlerResult::Undefined
}
