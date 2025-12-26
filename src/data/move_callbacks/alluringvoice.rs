//! Alluring Voice Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	alluringvoice: {
//! 		num: 914,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Special",
//! 		name: "Alluring Voice",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1, sound: 1, bypasssub: 1, metronome: 1 },
//! 		secondary: {
//! 			chance: 100,
//! 			onHit(target, source, move) {
//! 				if (target?.statsRaisedThisTurn) {
//! 					target.addVolatile('confusion', source, move);
//! 				}
//! 			},
//! 		},
//! 		target: "normal",
//! 		type: "Fairy",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit callback for Alluring Voice
/// Adds confusion if target raised stats this turn
pub fn on_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (target?.statsRaisedThisTurn) { target.addVolatile('confusion', source, move); }

    let stats_raised = if let Some(side) = battle.sides.get(target.0) {
        if let Some(pokemon) = side.pokemon.get(target.1) {
            pokemon.stats_raised_this_turn
        } else {
            false
        }
    } else {
        false
    };

    if stats_raised {
        battle.add_volatile_to_pokemon(
            target,
            &ID::new("confusion"),
            Some(source),
            Some(move_id),
        );
    }

    MoveHandlerResult::Undefined
}


