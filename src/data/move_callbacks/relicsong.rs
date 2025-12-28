//! Relic Song Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, pokemon, move) {
///     if (pokemon.baseSpecies.baseSpecies === 'Meloetta' && !pokemon.transformed) {
///         move.willChangeForme = true;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.baseSpecies.baseSpecies === 'Meloetta' && !pokemon.transformed) {
    //     move.willChangeForme = true;
    // }
    let (base_species, transformed) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.base_species.base_species.clone(), pokemon_pokemon.transformed)
    };

    if base_species == "Meloetta" && !transformed {
        let active_move = match &mut battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.will_change_forme = true;
    }

    EventResult::Continue
}

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (move.willChangeForme) {
///         const meloettaForme = pokemon.species_id.id === 'meloettapirouette' ? '' : '-Pirouette';
///         pokemon.formeChange('Meloetta' + meloettaForme, this.effect, false, '0', '[msg]');
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (move.willChangeForme) {
    let will_change_forme = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.will_change_forme
    };

    if will_change_forme {
        // const meloettaForme = pokemon.species_id.id === 'meloettapirouette' ? '' : '-Pirouette';
        let meloetta_forme = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            if pokemon_pokemon.species_id.id == ID::from("meloettapirouette") {
                ""
            } else {
                "-Pirouette"
            }
        };

        // pokemon.formeChange('Meloetta' + meloettaForme, this.effect, false, '0', '[msg]');
        let forme_name = format!("Meloetta{}", meloetta_forme);
        let effect_id = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.id.clone()
        };

        battle.forme_change(pokemon, &forme_name, Some(&effect_id), false, Some("0"), Some("[msg]"));
    }

    EventResult::Continue
}

