//! Electric Surge Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	electricsurge: {
//! 		onStart(source) {
//! 			this.field.setTerrain('electricterrain');
//! 		},
//! 		flags: {},
//! 		name: "Electric Surge",
//! 		rating: 4,
//! 		num: 226,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(source)
    pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
        battle.field.set_terrain(ID::new("electricterrain"), None);
        AbilityHandlerResult::Undefined
    }
