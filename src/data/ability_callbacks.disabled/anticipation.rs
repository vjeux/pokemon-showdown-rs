//! Anticipation Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	anticipation: {
//! 		onStart(pokemon) {
//! 			for (const target of pokemon.foes()) {
//! 				for (const moveSlot of target.moveSlots) {
//! 					const move = this.dex.moves.get(moveSlot.move);
//! 					if (move.category === 'Status') continue;
//! 					const moveType = move.id === 'hiddenpower' ? target.hpType : move.type;
//! 					if (
//! 						this.dex.getImmunity(moveType, pokemon) && this.dex.getEffectiveness(moveType, pokemon) > 0 ||
//! 						move.ohko
//! 					) {
//! 						this.add('-ability', pokemon, 'Anticipation');
//! 						return;
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Anticipation",
//! 		rating: 0.5,
//! 		num: 107,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::data::typechart::{get_effectiveness_multi, is_immune};
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const target of pokemon.foes())
    // Get the opposing side index (in singles, it's 0 or 1, the opposite of current)
    let foe_side_index = if pokemon.side_index == 0 { 1 } else { 0 };
    let foes = pokemon.foes_stub(foe_side_index, false);

    for (target_side_idx, target_pos) in foes {
        let target = &battle.sides[target_side_idx].pokemon[target_pos];

        // for (const moveSlot of target.moveSlots)
        for move_slot in &target.move_slots {
            // const move = this.dex.moves.get(moveSlot.move);
            let Some(move_data) = battle.dex.get_move(&move_slot.id.as_str()) else {
                continue;
            };

            // if (move.category === 'Status') continue;
            if move_data.category == "Status" {
                continue;
            }

            // const moveType = move.id === 'hiddenpower' ? target.hpType : move.type;
            // Note: hpType is not currently stored on Pokemon in Rust version,
            // so we just use the move type. Hidden Power type calculation from IVs
            // would be needed for full Gen 7 and below support.
            let move_type = &move_data.move_type;

            // if (this.dex.getImmunity(moveType, pokemon) && this.dex.getEffectiveness(moveType, pokemon) > 0 || move.ohko)
            // Check if any of the pokemon's types is immune to the move
            let mut has_immunity = false;
            for poke_type in &pokemon.types {
                if is_immune(move_type, poke_type) {
                    has_immunity = true;
                    break;
                }
            }

            let effectiveness = get_effectiveness_multi(move_type, &pokemon.types);

            if (!has_immunity && effectiveness > 1.0) || move_data.ohko {
                // this.add('-ability', pokemon, 'Anticipation');
                battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Anticipation")]);
                // return;
                return AbilityHandlerResult::Undefined;
            }
        }
    }

    AbilityHandlerResult::Undefined
}
