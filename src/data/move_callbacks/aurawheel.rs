//! Aura Wheel Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onTry(source) {
///     if (source.species.baseSpecies === 'Morpeko') {
///         return;
///     }
///     this.attrLastMove('[still]');
///     this.add('-fail', source, 'move: Aura Wheel');
///     this.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.");
///     return null;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the source pokemon
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the species data
    let species_id = &source.species_id;
    let species_data = match battle.dex.get_species_by_id(species_id) {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // if (source.species.baseSpecies === 'Morpeko') {
    //     return;
    // }
    let base_species = species_data.base_species.as_ref().unwrap_or(&species_data.name);
    if base_species == "Morpeko" {
        return EventResult::Continue;
    }

    // this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // Get source again for battle.add
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // this.add('-fail', source, 'move: Aura Wheel');
    battle.add("-fail", &[source.into(), "move: Aura Wheel".into()]);

    // this.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.");
    battle.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.", false, None);

    // return null;
    EventResult::Fail
}

/// onModifyType(move, pokemon) {
///     if (pokemon.species.name === 'Morpeko-Hangry') {
///         move.type = 'Dark';
///     } else {
///         move.type = 'Electric';
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the species data
    let species_id = &pokemon.species_id;
    let species_data = match battle.dex.get_species_by_id(species_id) {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // if (pokemon.species.name === 'Morpeko-Hangry') {
    //     move.type = 'Dark';
    // } else {
    //     move.type = 'Electric';
    // }
    if species_data.name == "Morpeko-Hangry" {
        EventResult::String("Dark".to_string())
    } else {
        EventResult::String("Electric".to_string())
    }
}

