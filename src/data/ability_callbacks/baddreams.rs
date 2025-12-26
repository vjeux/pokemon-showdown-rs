//! Bad Dreams Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	baddreams: {
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			if (!pokemon.hp) return;
//! 			for (const target of pokemon.foes()) {
//! 				if (target.status === 'slp' || target.hasAbility('comatose')) {
//! 					this.damage(target.baseMaxhp / 8, target, pokemon);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Bad Dreams",
//! 		rating: 1.5,
//! 		num: 123,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onResidualOrder: 28
pub const ON_RESIDUAL_ORDER: i32 = 28;

/// onResidualSubOrder: 2
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (!pokemon.hp) return;
    if pokemon.hp == 0 {
        return AbilityHandlerResult::Undefined;
    }

    // for (const target of pokemon.foes())
    let foe_side_index = if pokemon.side_index == 0 { 1 } else { 0 };
    let foes = pokemon.foes_stub(foe_side_index, false);

    for (target_side_idx, target_pos) in foes {
        let target = &battle.sides[target_side_idx].pokemon[target_pos];

        // if (target.status === 'slp' || target.hasAbility('comatose'))
        if target.status.as_str() == "slp" || target.ability.as_str() == "comatose" {
            // this.damage(target.baseMaxhp / 8, target, pokemon);
            let damage = target.base_maxhp / 8;
            battle.damage(damage as i32, Some((target_side_idx, target_pos)), Some((pokemon.side_index, pokemon.position)), None, false);
        }
    }

    AbilityHandlerResult::Undefined
}

