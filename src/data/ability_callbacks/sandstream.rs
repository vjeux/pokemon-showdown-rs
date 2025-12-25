//! Sand Stream Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sandstream: {
//! 		onStart(source) {
//! 			this.field.setWeather('sandstorm');
//! 		},
//! 		flags: {},
//! 		name: "Sand Stream",
//! 		rating: 4,
//! 		num: 45,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setWeather('sandstorm');
    battle.field.set_weather(ID::new("sandstorm"), None);
    AbilityHandlerResult::Undefined
}
