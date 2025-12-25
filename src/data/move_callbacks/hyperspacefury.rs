//! Hyperspace Fury Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	hyperspacefury: {
//! 		num: 621,
//! 		accuracy: true,
//! 		basePower: 100,
//! 		category: "Physical",
//! 		name: "Hyperspace Fury",
//! 		pp: 5,
//! 		priority: 0,
//! 		flags: { mirror: 1, bypasssub: 1, nosketch: 1 },
//! 		breaksProtect: true,
//! 		onTry(source) {
//! 			if (source.species.name === 'Hoopa-Unbound') {
//! 				return;
//! 			}
//! 			this.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");
//! 			if (source.species.name === 'Hoopa') {
//! 				this.attrLastMove('[still]');
//! 				this.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
//! 				return null;
//! 			}
//! 			this.attrLastMove('[still]');
//! 			this.add('-fail', source, 'move: Hyperspace Fury');
//! 			return null;
//! 		},
//! 		self: {
//! 			boosts: {
//! 				def: -1,
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Dark",
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(...)
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

