//! Aura Wheel Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	aurawheel: {
//! 		num: 783,
//! 		accuracy: 100,
//! 		basePower: 110,
//! 		category: "Physical",
//! 		name: "Aura Wheel",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1 },
//! 		secondary: {
//! 			chance: 100,
//! 			self: {
//! 				boosts: {
//! 					spe: 1,
//! 				},
//! 			},
//! 		},
//! 		onTry(source) {
//! 			if (source.species.baseSpecies === 'Morpeko') {
//! 				return;
//! 			}
//! 			this.attrLastMove('[still]');
//! 			this.add('-fail', source, 'move: Aura Wheel');
//! 			this.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.");
//! 			return null;
//! 		},
//! 		onModifyType(move, pokemon) {
//! 			if (pokemon.species.name === 'Morpeko-Hangry') {
//! 				move.type = 'Dark';
//! 			} else {
//! 				move.type = 'Electric';
//! 			}
//! 		},
//! 		target: "normal",
//! 		type: "Electric",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry callback for Aura Wheel
/// Only Morpeko (or Morpeko-Hangry) can use this move
/// JS: onTry(source) { ... }
pub fn on_try(
    battle: &mut Battle,
    source: (usize, usize),
    target: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (source.species.baseSpecies === 'Morpeko') { return; }
    let (side_idx, poke_idx) = source;
    let base_species = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            // Extract base species from species_id (e.g., "morpekohangry" -> "morpeko")
            let species = pokemon.species_id.as_str();
            // Since species_id is lowercase and without hyphens, check if it starts with "morpeko"
            if species.starts_with("morpeko") {
                "morpeko"
            } else {
                species
            }
        } else {
            return MoveHandlerResult::Null;
        }
    } else {
        return MoveHandlerResult::Null;
    };

    // Check if base species is Morpeko (case-insensitive since ID is lowercase)
    if base_species == "morpeko" {
        // Move can be used
        return MoveHandlerResult::Undefined;
    }

    // JavaScript: this.attrLastMove('[still]');
    // TODO: Implement attrLastMove

    // JavaScript: this.add('-fail', source, 'move: Aura Wheel');
    let source_id = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            format!("{}: {}", side.id_str(), pokemon.name)
        } else {
            String::from("unknown")
        }
    } else {
        String::from("unknown")
    };
    battle.add_log("-fail", &[&source_id, "move: Aura Wheel"]);

    // JavaScript: this.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.");
    battle.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.", false, None);

    // JavaScript: return null;
    MoveHandlerResult::Null
}

/// onModifyType callback for Aura Wheel
/// Changes move type to Dark if user is Morpeko-Hangry, Electric otherwise
/// JS: onModifyType(move, pokemon) { ... }
pub fn on_modify_type(
    battle: &mut Battle,
    user: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (pokemon.species.name === 'Morpeko-Hangry') { move.type = 'Dark'; } else { move.type = 'Electric'; }
    let (side_idx, poke_idx) = user;
    let species_id = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            pokemon.species_id.as_str().to_string()
        } else {
            return MoveHandlerResult::Undefined;
        }
    } else {
        return MoveHandlerResult::Undefined;
    };

    // Set move type based on Morpeko form
    // species_id for "Morpeko-Hangry" would be "morpekohangry"
    if species_id == "morpekohangry" {
        // TODO: Actually modify the active move's type to Dark
        // This requires storing the active move somewhere modifiable
        // For now, just return Undefined - the type modification infrastructure isn't built yet
        MoveHandlerResult::Undefined
    } else {
        // Default type is Electric
        MoveHandlerResult::Undefined
    }
}
