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
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.baseSpecies.baseSpecies === 'Meloetta' && !pokemon.transformed) {
    //     move.willChangeForme = true;
    // }
    let (base_species, transformed) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            pokemon_pokemon.base_species.clone(),
            pokemon_pokemon.transformed,
        )
    };

    if base_species == ID::from("meloetta") && !transformed {
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
pub fn on_after_move_secondary_self(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (move.willChangeForme) {
    let will_change_forme = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
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
            if pokemon_pokemon.species_id == ID::from("meloettapirouette") {
                ""
            } else {
                "-Pirouette"
            }
        };

        // pokemon.formeChange('Meloetta' + meloettaForme, this.effect, false, '0', '[msg]');
        let _forme_name = format!("Meloetta{}", meloetta_forme);
        let _effect_id = {
            let active_move = match &battle.active_move {
                Some(active_move) => &active_move.id,
                None => return EventResult::Continue,
            };
            active_move.clone()
        };

        // TODO: Implement forme_change method in Battle
        // battle.forme_change(...);
    }

    EventResult::Continue
}
