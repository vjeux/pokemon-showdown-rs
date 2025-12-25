//! Drizzle Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	drizzle: {
//! 		onStart(source) {
//! 			if (source.species.id === 'kyogre' && source.item === 'blueorb') return;
//! 			this.field.setWeather('raindance');
//! 		},
//! 		flags: {},
//! 		name: "Drizzle",
//! 		rating: 4,
//! 		num: 2,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
    pub fn on_start(battle: &mut Battle, source: &Pokemon) -> AbilityHandlerResult {
        // If Kyogre with Blue Orb, don't set weather (Primal Reversion handles it)
        if source.species_id == ID::new("kyogre") && source.item == ID::new("blueorb") {
            return AbilityHandlerResult::Undefined;
        }
        battle.field.set_weather(ID::new("raindance"), None);
        AbilityHandlerResult::Undefined
    }
