//! Zero to Hero Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchOut(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
///     if (pokemon.species.forme !== 'Hero') {
///         pokemon.formeChange('Palafin-Hero', this.effect, true);
///         pokemon.heroMessageDisplayed = false;
///     }
/// }
pub fn on_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
    let (base_species_base_species, forme): (Option<String>, Option<String>) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (base_species_base_species, species_data.forme.clone())
    };

    if base_species_base_species.as_deref() != Some("Palafin") {
        return EventResult::Continue;
    }

    // if (pokemon.species.forme !== 'Hero')
    if forme.as_deref() != Some("Hero") {
        // pokemon.formeChange('Palafin-Hero', this.effect, true);
        unsafe {
            let battle_ptr = battle as *mut Battle;
            let battle_ref1 = &mut *battle_ptr;
            let battle_ref2 = &mut *battle_ptr;

            let side = &mut battle_ref1.sides[pokemon_pos.0];
            let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
            if let Some(pokemon_index) = active_slot {
                if pokemon_index < side.pokemon.len() {
                    let pokemon = &mut side.pokemon[pokemon_index];
                    pokemon.forme_change(battle_ref2, ID::from("palafinhero"), Some(ID::from("zerotohero")), true, "0", None);
                }
            }
        }

        // pokemon.heroMessageDisplayed = false;
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.hero_message_displayed = false;
        }
    }

    EventResult::Continue
}

/// onSwitchIn(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
///     if (!pokemon.heroMessageDisplayed && pokemon.species.forme === 'Hero') {
///         this.add('-activate', pokemon, 'ability: Zero to Hero');
///         pokemon.heroMessageDisplayed = true;
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // if (pokemon.baseSpecies.baseSpecies !== 'Palafin') return;
    let (base_species_base_species, forme): (Option<String>, Option<String>) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (base_species_base_species, species_data.forme.clone())
    };

    if base_species_base_species.as_deref() != Some("Palafin") {
        return EventResult::Continue;
    }

    // if (!pokemon.heroMessageDisplayed && pokemon.species.forme === 'Hero')
    let hero_message_displayed = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hero_message_displayed
    };

    if !hero_message_displayed && forme.as_deref() == Some("Hero") {
        // this.add('-activate', pokemon, 'ability: Zero to Hero');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Zero to Hero"),
        ]);

        // pokemon.heroMessageDisplayed = true;
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.hero_message_displayed = true;
        }
    }

    EventResult::Continue
}

