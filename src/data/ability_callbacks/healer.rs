//! Healer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     for (const allyActive of pokemon.adjacentAllies()) {
///         if (allyActive.status && this.randomChance(3, 10)) {
///             this.add('-activate', pokemon, 'ability: Healer');
///             allyActive.cureStatus();
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::pokemon::Pokemon;
    use crate::battle::Arg;

    // for (const allyActive of pokemon.adjacentAllies())
    let adjacent_allies = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.adjacent_allies(battle)
    };

    for ally_pos in adjacent_allies {
        // if (allyActive.status && this.randomChance(3, 10))
        let has_status = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            !ally.status.is_empty()
        };

        if has_status && battle.random_chance(3, 10) {
            let pokemon_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let side_id = format!("p{}", pokemon.side_index + 1);
                if pokemon.is_active {
                    let pos_letter = (b'a' + pokemon.position as u8) as char;
                    format!("{}{}: {}", side_id, pos_letter, pokemon.name)
                } else {
                    format!("{}: {}", side_id, pokemon.name)
                }
            };

            battle.add("-activate", &[
                Arg::String(pokemon_id),
                Arg::Str("ability: Healer"),
            ]);

            Pokemon::cure_status(battle, ally_pos, false);
        }
    }

    EventResult::Continue
}

