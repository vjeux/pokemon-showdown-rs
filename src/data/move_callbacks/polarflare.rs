//! Polar Flare Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onHit(target, pokemon, move) {
///     if (pokemon.baseSpecies.baseSpecies === 'Ramnarok' && !pokemon.transformed) {
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

    // if (pokemon.baseSpecies.baseSpecies === 'Ramnarok' && !pokemon.transformed) {
    let (is_ramnarok, transformed) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            pokemon_pokemon.base_species == ID::from("ramnarok"),
            pokemon_pokemon.transformed,
        )
    };

    if is_ramnarok && !transformed {
        // move.willChangeForme = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.will_change_forme = true;
        }
    }

    EventResult::Continue
}

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (move.willChangeForme) {
///         const forme = pokemon.species_id.id === 'ramnarokradiant' ? '' : '-Radiant';
///         pokemon.formeChange('Ramnarok' + forme, this.effect, false, '0', '[msg]');
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

    if !will_change_forme {
        return EventResult::Continue;
    }

    // const forme = pokemon.species_id.id === 'ramnarokradiant' ? '' : '-Radiant';
    let forme = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if pokemon_pokemon.species_id == ID::from("ramnarokradiant") {
            ""
        } else {
            "-Radiant"
        }
    };

    // pokemon.formeChange('Ramnarok' + forme, this.effect, false, '0', '[msg]');
    let target_forme = format!("Ramnarok{}", forme);
    let effect_id = {
        match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        }
    };

    // Use position-based forme_change
    crate::pokemon::Pokemon::forme_change(
        battle,
        pokemon,
        ID::from(target_forme.as_str()),
        Some(Effect::move_(effect_id)),
        false,
        "0",
        Some("[msg]"),
    );

    EventResult::Continue
}
