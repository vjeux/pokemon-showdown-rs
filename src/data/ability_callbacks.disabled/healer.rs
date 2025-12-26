//! Healer Ability - Heals ally status in doubles
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! ```text
//! JS Source (data/abilities.ts):
//! healer: {
//!     onResidualOrder: 5,
//!     onResidualSubOrder: 3,
//!     onResidual(pokemon) {
//!       for (const allyActive of pokemon.adjacentAllies()) {
//!         if (allyActive.status && this.randomChance(3, 10)) {
//!           this.add("-activate", pokemon, "ability: Healer");
//!           allyActive.cureStatus();
//!         }
//!       }
//!     },
//!     flags: {},
//!     name: "Healer",
//!     rating: 0,
//!     num: 131
//!   },
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 5;
pub const ON_RESIDUAL_SUB_ORDER: i32 = 3;

/// onResidual(pokemon)
/// 30% chance to cure status of adjacent allies
pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const allyActive of pokemon.adjacentAllies())
    let side_index = pokemon.side_index;

    // Collect adjacent allies with status conditions (two-phase pattern)
    let mut allies_with_status: Vec<(usize, String)> = Vec::new();
    if let Some(side) = battle.sides.get(side_index) {
        for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // Skip self
            if ally.position == pokemon.position {
                continue;
            }
            // if (allyActive.status)
            if !ally.status.is_empty() {
                allies_with_status.push((ally.position, ally.name.clone()));
            }
        }
    }

    // Check random chance and heal
    for (ally_pos, _ally_name) in allies_with_status {
        // this.randomChance(3, 10)
        if battle.random_chance(3, 10) {
            // this.add("-activate", pokemon, "ability: Healer");
            battle.add("-activate", &[
                Arg::Pokemon(pokemon),
                Arg::Str("ability: Healer")
            ]);

            // allyActive.cureStatus();
            battle.sides[side_index].pokemon[ally_pos].cure_status();
        }
    }

    AbilityHandlerResult::Undefined
}
