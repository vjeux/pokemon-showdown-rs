//! Sand Spit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sandspit: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			this.field.setWeather('sandstorm');
//! 		},
//! 		flags: {},
//! 		name: "Sand Spit",
//! 		rating: 1,
//! 		num: 245,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Sets sandstorm when hit by a damaging move
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, _target: &Pokemon, _source: &mut Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // this.field.setWeather('sandstorm');
    battle.field.set_weather(ID::new("sandstorm"), None);
    AbilityHandlerResult::Undefined
}

