//! Mimicry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onStart(pokemon) {
///     this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // NOTE: singleEvent('TerrainChange') skipped - requires event system infrastructure
    // this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
    EventResult::Continue
}

/// onTerrainChange(pokemon) {
///     let types;
///     switch (this.field.terrain) {
///     case 'electricterrain':
///         types = ['Electric'];
///         break;
///     case 'grassyterrain':
///         types = ['Grass'];
///         break;
///     case 'mistyterrain':
///         types = ['Fairy'];
///         break;
///     case 'psychicterrain':
///         types = ['Psychic'];
///         break;
///     default:
///         types = pokemon.baseSpecies.types;
///     }
///     const oldTypes = pokemon.getTypes();
///     if (oldTypes.join() === types.join() || !pokemon.setType(types)) return;
///     if (this.field.terrain || pokemon.transformed) {
///         this.add('-start', pokemon, 'typechange', types.join('/'), '[from] ability: Mimicry');
///         if (!this.field.terrain) this.hint("Transform Mimicry changes you to your original un-transformed types.");
///     } else {
///         this.add('-activate', pokemon, 'ability: Mimicry');
///         this.add('-end', pokemon, 'typechange', '[silent]');
///     }
/// }
pub fn on_terrain_change(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // Determine new types based on terrain
    let types = {
        let terrain = battle.field.get_terrain().as_str();

        match terrain {
            "electricterrain" => vec!["Electric".to_string()],
            "grassyterrain" => vec!["Grass".to_string()],
            "mistyterrain" => vec!["Fairy".to_string()],
            "psychicterrain" => vec!["Psychic".to_string()],
            _ => {
                // default: types = pokemon.baseSpecies.types;
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                match battle.dex.species().get(pokemon.base_species.as_str()) {
                    Some(species) => species.types.clone(),
                    None => return EventResult::Continue,
                }
            }
        }
    };

    // const oldTypes = pokemon.getTypes();
    let (old_types, transformed) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.get_types(battle, false), pokemon.transformed)
    };

    // if (oldTypes.join() === types.join() || !pokemon.setType(types)) return;
    if old_types.join(",") == types.join(",") {
        return EventResult::Continue;
    }

    let success = Pokemon::set_type(battle, pokemon_pos, types.clone(), false);
    if !success {
        return EventResult::Continue;
    }

    // Get pokemon slot for messages
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // Check if terrain is active
    // In JavaScript: if (this.field.terrain || pokemon.transformed)
    let terrain_active = !battle.field.terrain.is_empty();

    // if (this.field.terrain || pokemon.transformed)
    if terrain_active || transformed {
        // this.add('-start', pokemon, 'typechange', types.join('/'), '[from] ability: Mimicry');
        battle.add("-start", &[
            pokemon_slot.as_str().into(),
            "typechange".into(),
            types.join("/").into(),
            "[from] ability: Mimicry".into(),
        ]);

        // if (!this.field.terrain) this.hint("Transform Mimicry changes you to your original un-transformed types.");
        // NOTE: battle.hint() not available - skipping hint message
    } else {
        // this.add('-activate', pokemon, 'ability: Mimicry');
        battle.add("-activate", &[
            pokemon_slot.as_str().into(),
            "ability: Mimicry".into(),
        ]);

        // this.add('-end', pokemon, 'typechange', '[silent]');
        battle.add("-end", &[
            pokemon_slot.as_str().into(),
            "typechange".into(),
            "[silent]".into(),
        ]);
    }

    EventResult::Continue
}

