//! Relic Song Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
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
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.willChangeForme) {
    if !active_move_ref.will_change_forme {
        return EventResult::Continue;
    }

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
    let forme_name = format!("Meloetta{}", meloetta_forme);
    let effect_id = active_move_ref.id.clone();

    // Use position-based forme_change
    crate::pokemon::Pokemon::forme_change(
        battle,
        pokemon,
        ID::from(forme_name.as_str()),
        Some(Effect::move_(effect_id)),
        false,
        "0",
        Some("[msg]"),
    );

    EventResult::Continue
}
