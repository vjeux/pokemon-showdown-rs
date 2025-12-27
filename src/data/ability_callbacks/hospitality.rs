//! Hospitality Ability - Heals ally HP on switch-in
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hospitality: {
//! 		onStart(pokemon) {
//! 			for (const ally of pokemon.adjacentAllies()) {
//! 				this.heal(ally.maxhp / 4, ally);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Hospitality",
//! 		rating: 0,
//! 		num: 299,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Heals adjacent allies by 1/4 of their max HP
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const ally of pokemon.adjacentAllies())
    let side_index = pokemon.side_index;

    // Collect adjacent ally positions first to avoid borrow checker issues
    let mut allies_to_heal: Vec<(usize, i32)> = Vec::new(); // (position, heal_amount)

    if let Some(side) = battle.sides.get(side_index) {
        for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // Skip self
            if ally.position == pokemon.position {
                continue;
            }
            // In doubles, allies are adjacent if they're on the same side and active
            // this.heal(ally.maxhp / 4, ally);
            let heal_amount = ally.maxhp / 4;
            allies_to_heal.push((ally.position, heal_amount));
        }
    }

    // Now heal allies
    for (position, heal_amount) in allies_to_heal {
        battle.heal(heal_amount as i32, Some((side_index, position)), Some((pokemon.side_index, pokemon.position)), None);
    }

    AbilityHandlerResult::Undefined
}

