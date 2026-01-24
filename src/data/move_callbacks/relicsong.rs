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
/// NOTE: dispatch_on_hit calls on_hit(battle, target_pos, source_pos)
/// JS signature: onHit(target, pokemon/source, move)
/// So first param is target, second param is source (the one using the move)
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // pokemon is the SOURCE (user of the move), not target
    let pokemon = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (pokemon.baseSpecies.baseSpecies === 'Meloetta' && !pokemon.transformed) {
    //     move.willChangeForme = true;
    // }
    // JavaScript: pokemon.baseSpecies is the Species object for the Pokemon's current species
    // JavaScript: pokemon.baseSpecies.baseSpecies is the base species NAME (e.g., "Meloetta" for Meloetta-Pirouette)
    let (species_base_species, transformed) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Look up the species data to get the true base species
        // JavaScript: pokemon.baseSpecies.baseSpecies (species object's baseSpecies property)
        let base_species_name = battle.dex.species()
            .get(pokemon_pokemon.base_species.as_str())
            .and_then(|species_data| species_data.base_species.as_ref())
            .map(|bs| bs.to_lowercase())
            .unwrap_or_else(|| pokemon_pokemon.base_species.to_string());

        (
            base_species_name,
            pokemon_pokemon.transformed,
        )
    };

    if species_base_species == "meloetta" && !transformed {
        let active_move = match &mut battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.borrow_mut().will_change_forme = true;
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
    let move_effect = battle.make_move_effect(&active_move_ref.id);

    // Use position-based forme_change
    crate::pokemon::Pokemon::forme_change(
        battle,
        pokemon,
        ID::from(forme_name.as_str()),
        Some(move_effect),
        false,
        "0",
        Some("[msg]"),
    );

    EventResult::Continue
}
