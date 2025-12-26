//! Intimidate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	intimidate: {
//! 		onStart(pokemon) {
//! 			let activated = false;
//! 			for (const target of pokemon.adjacentFoes()) {
//! 				if (!activated) {
//! 					this.add('-ability', pokemon, 'Intimidate', 'boost');
//! 					activated = true;
//! 				}
//! 				if (target.volatiles['substitute']) {
//! 					this.add('-immune', target);
//! 				} else {
//! 					this.boost({ atk: -1 }, target, pokemon, null, true);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Intimidate",
//! 		rating: 3.5,
//! 		num: 22,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Lowers Attack of adjacent foes when entering battle
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    let mut activated = false;

    // Get all active Pokemon and filter for foes on the opposite side
    // In the simple case (activePerHalf <= 2), all foes are adjacent
    // For doubles+ (activePerHalf > 2), would need to check adjacency properly
    // For now, implementing the basic case

    let pokemon_side = pokemon.side_index;
    let pokemon_position = pokemon.position;
    let foe_side = if pokemon_side == 0 { 1 } else { 0 };

    // Get all active Pokemon
    let all_active = battle.get_all_active(false);

    // Collect foe targets along with substitute status (need to collect first to avoid borrow issues)
    let foe_targets: Vec<(usize, usize, bool)> = all_active.iter()
        .filter(|foe_pokemon| foe_pokemon.side_index == foe_side)
        .map(|foe_pokemon| {
            let has_substitute = foe_pokemon.volatiles.contains_key(&ID::from("substitute"));
            (foe_pokemon.side_index, foe_pokemon.position, has_substitute)
        })
        .collect();

    for (target_side, target_idx, has_substitute) in foe_targets {
        if !activated {
            // this.add('-ability', pokemon, 'Intimidate', 'boost');
            battle.add("-ability", &[
                Arg::Pokemon(pokemon),
                Arg::Str("Intimidate"),
                Arg::Str("boost")
            ]);
            activated = true;
        }

        if has_substitute {
            // this.add('-immune', target);
            // Temporarily get reference and immediately use it
            if let Some(side) = battle.sides.get(target_side) {
                if let Some(target_pokemon) = side.pokemon.get(target_idx) {
                    // Copy the pokemon name to avoid borrow issues
                    let pokemon_name = target_pokemon.name.clone();
                    let pokemon_side_idx = target_pokemon.side_index;
                    let pokemon_pos = target_pokemon.position;
                    drop(target_pokemon);
                    drop(side);

                    // Now we can call battle.add with a mutable borrow
                    // Use a string-based arg instead of Pokemon reference
                    battle.add("-immune", &[Arg::Str(&format!("p{}a: {}", pokemon_side_idx + 1, pokemon_name))]);
                }
            }
        } else {
            // this.boost({ atk: -1 }, target, pokemon, null, true);
            battle.boost(&[("atk", -1)], (target_side, target_idx), Some((pokemon_side, pokemon_position)), None);
        }
    }

    AbilityHandlerResult::Undefined
}

