//! Schooling Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed) return;
///     if (pokemon.hp > pokemon.maxhp / 4) {
///         if (pokemon.species.id === 'wishiwashi') {
///             pokemon.formeChange('Wishiwashi-School');
///         }
///     } else {
///         if (pokemon.species.id === 'wishiwashischool') {
///             pokemon.formeChange('Wishiwashi');
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed) return;
    let (base_species_base_species, level, transformed, hp, maxhp, species_id): (Option<String>, u8, bool, i32, i32, ID) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        (
            base_species_base_species,
            pokemon.level,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            pokemon.species_id.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Wishiwashi") || level < 20 || transformed {
        return EventResult::Continue;
    }

    // if (pokemon.hp > pokemon.maxhp / 4)
    if hp > maxhp / 4 {
        // if (pokemon.species.id === 'wishiwashi')
        if species_id.as_str() == "wishiwashi" {
            // pokemon.formeChange('Wishiwashi-School');
            // pokemon_pos is already (side_idx, pokemon_index), pass it directly
            crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("wishiwashischool"), Some(battle.make_ability_effect(&ID::from("schooling"))), false, "0", None);
        }
    } else {
        // if (pokemon.species.id === 'wishiwashischool')
        if species_id.as_str() == "wishiwashischool" {
            // pokemon.formeChange('Wishiwashi');
            // pokemon_pos is already (side_idx, pokemon_index), pass it directly
            crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("wishiwashi"), Some(battle.make_ability_effect(&ID::from("schooling"))), false, "0", None);
        }
    }

    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (
///         pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 ||
///         pokemon.transformed || !pokemon.hp
///     ) return;
///     if (pokemon.hp > pokemon.maxhp / 4) {
///         if (pokemon.species.id === 'wishiwashi') {
///             pokemon.formeChange('Wishiwashi-School');
///         }
///     } else {
///         if (pokemon.species.id === 'wishiwashischool') {
///             pokemon.formeChange('Wishiwashi');
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed || !pokemon.hp) return;
    let (base_species_base_species, level, transformed, hp, maxhp, species_id): (Option<String>, u8, bool, i32, i32, ID) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        (
            base_species_base_species,
            pokemon.level,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            pokemon.species_id.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Wishiwashi") || level < 20 || transformed || hp == 0 {
        return EventResult::Continue;
    }

    // if (pokemon.hp > pokemon.maxhp / 4)
    if hp > maxhp / 4 {
        // if (pokemon.species.id === 'wishiwashi')
        if species_id.as_str() == "wishiwashi" {
            // pokemon.formeChange('Wishiwashi-School');
            // pokemon_pos is already (side_idx, pokemon_index), pass it directly
            crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("wishiwashischool"), Some(battle.make_ability_effect(&ID::from("schooling"))), false, "0", None);
        }
    } else {
        // if (pokemon.species.id === 'wishiwashischool')
        if species_id.as_str() == "wishiwashischool" {
            // pokemon.formeChange('Wishiwashi');
            // pokemon_pos is already (side_idx, pokemon_index), pass it directly
            crate::pokemon::Pokemon::forme_change(battle, pokemon_pos, ID::from("wishiwashi"), Some(battle.make_ability_effect(&ID::from("schooling"))), false, "0", None);
        }
    }

    EventResult::Continue
}

