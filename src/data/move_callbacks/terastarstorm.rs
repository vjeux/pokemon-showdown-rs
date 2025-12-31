//! Tera Starstorm Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.species_id.as_str() === 'Terapagos-Stellar') {
///         move.type = 'Stellar';
///         if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) {
///             move.category = 'Physical';
///         }
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::StatID;

    let pokemon = pokemon_pos;

    // if (pokemon.species_id.as_str() === 'Terapagos-Stellar')
    let (is_terapagos_stellar, should_be_physical) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let is_terapagos = pokemon_ref.species_id.as_str() == "Terapagos-Stellar";

        if is_terapagos {
            // if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true))
            let should_be_physical = if pokemon_ref.terastallized.is_some() {
                let atk_stat = battle.get_pokemon_stat(pokemon, StatID::Atk, false, true);
                let spa_stat = battle.get_pokemon_stat(pokemon, StatID::SpA, false, true);
                atk_stat > spa_stat
            } else {
                false
            };

            (true, should_be_physical)
        } else {
            (false, false)
        }
    };

    if is_terapagos_stellar {
        if let Some(ref mut active_move) = battle.active_move {
            // move.type = 'Stellar';
            active_move.move_type = "Stellar".to_string();

            // if (pokemon.terastallized && ...)
            if should_be_physical {
                // move.category = 'Physical';
                active_move.category = "Physical".to_string();
            }
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.species_id.as_str() === 'Terapagos-Stellar') {
///         move.target = 'allAdjacentFoes';
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.species_id.as_str() === 'Terapagos-Stellar')
    let is_terapagos_stellar = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon_ref.species_id.as_str() == "Terapagos-Stellar"
    };

    if is_terapagos_stellar {
        if let Some(ref mut active_move) = battle.active_move {
            // move.target = 'allAdjacentFoes';
            active_move.target = "allAdjacentFoes".to_string();
        }
    }

    EventResult::Continue
}
