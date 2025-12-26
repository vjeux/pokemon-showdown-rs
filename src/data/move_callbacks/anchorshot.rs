//! Anchor Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	anchorshot: {
//! 		num: 677,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Physical",
//! 		isNonstandard: "Past",
//! 		name: "Anchor Shot",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1 },
//! 		secondary: {
//! 			chance: 100,
//! 			onHit(target, source, move) {
//! 				if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');
//! 			},
//! 		},
//! 		target: "normal",
//! 		type: "Steel",
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit callback for Anchor Shot
/// Traps the target if source is still active
pub fn on_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');

    // Check if source is active
    let source_is_active = if let Some(side) = battle.sides.get(source.0) {
        if let Some(pokemon) = side.pokemon.get(source.1) {
            pokemon.is_active
        } else {
            false
        }
    } else {
        false
    };

    if source_is_active {
        // Add trapped volatile
        // TODO: Support linkedStatus parameter ('trapper' in JS)
        // For now, just add the trapped volatile
        battle.add_volatile_to_pokemon(
            target,
            &ID::new("trapped"),
            Some(source),
            Some(move_id),
        );
    }

    MoveHandlerResult::Undefined
}

