//! Screen Cleaner Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	screencleaner: {
//! 		onStart(pokemon) {
//! 			let activated = false;
//! 			for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil']) {
//! 				for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()]) {
//! 					if (side.getSideCondition(sideCondition)) {
//! 						if (!activated) {
//! 							this.add('-activate', pokemon, 'ability: Screen Cleaner');
//! 							activated = true;
//! 						}
//! 						side.removeSideCondition(sideCondition);
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Screen Cleaner",
//! 		rating: 2,
//! 		num: 251,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Removes screens (Reflect, Light Screen, Aurora Veil) from both sides
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // let activated = false;
    let mut activated = false;

    // for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil'])
    let side_conditions = ["reflect", "lightscreen", "auroraveil"];

    for side_condition in side_conditions {
        let condition_id = ID::new(side_condition);

        // for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()])
        // Check both the pokemon's side and all foe sides
        for side_idx in 0..battle.sides.len() {
            // if (side.getSideCondition(sideCondition))
            if battle.sides[side_idx].has_side_condition(&condition_id) {
                // if (!activated)
                if !activated {
                    // this.add('-activate', pokemon, 'ability: Screen Cleaner');
                    battle.add("-activate", &[
                        Arg::Pokemon(pokemon),
                        Arg::Str("ability: Screen Cleaner")
                    ]);
                    // activated = true;
                    activated = true;
                }
                // side.removeSideCondition(sideCondition);
                battle.sides[side_idx].remove_side_condition(&condition_id);
            }
        }
    }

    AbilityHandlerResult::Undefined
}

