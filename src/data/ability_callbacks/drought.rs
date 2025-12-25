//! Drought Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	drought: {
//! 		onStart(source) {
//! 			if (source.species.id === 'groudon' && source.item === 'redorb') return;
//! 			this.field.setWeather('sunnyday');
//! 		},
//! 		flags: {},
//! 		name: "Drought",
//! 		rating: 4,
//! 		num: 70,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
    pub fn on_start(battle: &mut Battle, source: &Pokemon) -> AbilityHandlerResult {
        // If Groudon with Red Orb, don't set weather (Primal Reversion handles it)
        if source.species_id == ID::new("groudon") && source.item == ID::new("redorb") {
            return AbilityHandlerResult::Undefined;
        }
        battle.field.set_weather(ID::new("sunnyday"), None);
        AbilityHandlerResult::Undefined
    }
