//! Curious Medicine Ability - Clears ally stat changes on switch-in
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! ```text
//! JS Source (data/abilities.ts):
//! curiousmedicine: {
//!     onStart(pokemon) {
//!         for (const ally of pokemon.adjacentAllies()) {
//!             ally.clearBoosts();
//!             this.add('-clearboost', ally, '[from] ability: Curious Medicine', '[of] ' + pokemon);
//!         }
//!     },
//! },
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Clears all stat boosts from adjacent allies
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const ally of pokemon.adjacentAllies())
    let side_index = pokemon.side_index;

    // Collect allies that need boosts cleared with their names
    let mut allies_to_clear: Vec<(usize, String)> = Vec::new();

    if let Some(side) = battle.sides.get(side_index) {
        for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // Skip self
            if ally.position == pokemon.position {
                continue;
            }
            // In doubles, allies are adjacent if they're on the same side and active
            allies_to_clear.push((ally.position, ally.name.clone()));
        }
    }

    // Now clear boosts for allies
    for (position, ally_name) in allies_to_clear {
        // ally.clearBoosts();
        battle.sides[side_index].pokemon[position].clear_boosts();

        // this.add('-clearboost', ally, '[from] ability: Curious Medicine', '[of] ' + pokemon);
        battle.add("-clearboost", &[
            Arg::String(ally_name),
            Arg::Str("[from] ability: Curious Medicine"),
            Arg::String(format!("[of] {}", pokemon.name))
        ]);
    }

    AbilityHandlerResult::Undefined
}
