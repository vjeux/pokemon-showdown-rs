//! Minus Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	minus: {
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(spa, pokemon) {
//! 			for (const allyActive of pokemon.allies()) {
//! 				if (allyActive.hasAbility(['minus', 'plus'])) {
//! 					return this.chainModify(1.5);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Minus",
//! 		rating: 0,
//! 		num: 58,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(spa, pokemon)
/// Boosts SpA by 1.5x if ally has Plus or Minus
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const allyActive of pokemon.allies())
    let side = &battle.sides[pokemon.side_index];

    for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
        // Skip self
        if ally.position == pokemon.position {
            continue;
        }

        // if (allyActive.hasAbility(['minus', 'plus']))
        if ally.ability.as_str() == "minus" || ally.ability.as_str() == "plus" {
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
    }

    AbilityHandlerResult::Undefined
}

