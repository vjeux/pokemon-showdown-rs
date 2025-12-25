//! Light That Burns the Sky Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	lightthatburnsthesky: {
//! 		num: 723,
//! 		accuracy: true,
//! 		basePower: 200,
//! 		category: "Special",
//! 		isNonstandard: "Past",
//! 		name: "Light That Burns the Sky",
//! 		pp: 1,
//! 		priority: 0,
//! 		flags: {},
//! 		onModifyMove(move, pokemon) {
//! 			if (pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) move.category = 'Physical';
//! 		},
//! 		ignoreAbility: true,
//! 		isZ: "ultranecroziumz",
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Psychic",
//! 		contestType: "Cool",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyMove(...)
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

