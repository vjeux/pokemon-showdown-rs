//! Zen Mode Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Darmanitan' || pokemon.transformed) {
///         return;
///     }
///     if (pokemon.hp <= pokemon.maxhp / 2 && !['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
///         pokemon.addVolatile('zenmode');
///     } else if (pokemon.hp > pokemon.maxhp / 2 && ['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
///         pokemon.addVolatile('zenmode'); // in case of base Darmanitan-Zen
///         pokemon.removeVolatile('zenmode');
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::Pokemon;

    // if (pokemon.baseSpecies.baseSpecies !== 'Darmanitan' || pokemon.transformed)
    let (base_species_base_species, transformed, hp, maxhp, forme): (Option<String>, bool, i32, i32, Option<String>) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (
            base_species_base_species,
            pokemon.transformed,
            pokemon.hp,
            pokemon.maxhp,
            species_data.forme.clone(),
        )
    };

    if base_species_base_species.as_deref() != Some("Darmanitan") || transformed {
        return EventResult::Continue;
    }

    // if (pokemon.hp <= pokemon.maxhp / 2 && !['Zen', 'Galar-Zen'].includes(pokemon.species.forme))
    if hp <= maxhp / 2 && forme.as_deref() != Some("Zen") && forme.as_deref() != Some("Galar-Zen") {
        // pokemon.addVolatile('zenmode');
        Pokemon::add_volatile(battle, pokemon_pos, ID::from("zenmode"), Some(pokemon_pos), None, None);
    } else if hp > maxhp / 2 && (forme.as_deref() == Some("Zen") || forme.as_deref() == Some("Galar-Zen")) {
        // pokemon.addVolatile('zenmode'); // in case of base Darmanitan-Zen
        Pokemon::add_volatile(battle, pokemon_pos, ID::from("zenmode"), Some(pokemon_pos), None, None);
        // pokemon.removeVolatile('zenmode');
        Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("zenmode"));
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (!pokemon.volatiles['zenmode'] || !pokemon.hp) return;
///     pokemon.transformed = false;
///     delete pokemon.volatiles['zenmode'];
///     if (pokemon.species.baseSpecies === 'Darmanitan' && pokemon.species.battleOnly) {
///         pokemon.formeChange(pokemon.species.battleOnly as string, this.effect, false, '0', '[silent]');
///     }
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::Pokemon;

    // if (!pokemon.volatiles['zenmode'] || !pokemon.hp) return;
    let (has_zenmode, hp, base_species, battle_only): (bool, i32, Option<String>, Option<String>) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_zenmode = pokemon.has_volatile(&ID::from("zenmode"));

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        // Extract first string from StringOrVec
        let battle_only_str = match &species_data.battle_only {
            crate::dex::StringOrVec::Single(s) => Some(s.clone()),
            crate::dex::StringOrVec::Multiple(v) if !v.is_empty() => Some(v[0].clone()),
            _ => None,
        };

        (has_zenmode, pokemon.hp, species_data.base_species.clone(), battle_only_str)
    };

    if !has_zenmode || hp == 0 {
        return EventResult::Continue;
    }

    // pokemon.transformed = false;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.transformed = false;
    }

    // delete pokemon.volatiles['zenmode'];
    Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("zenmode"));

    // if (pokemon.species.baseSpecies === 'Darmanitan' && pokemon.species.battleOnly)
    if base_species.as_deref() == Some("Darmanitan") {
        if let Some(battle_only_species) = battle_only {
            // pokemon.formeChange(pokemon.species.battleOnly as string, this.effect, false, '0', '[silent]');
            unsafe {
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        let pokemon = &mut side.pokemon[pokemon_index];
                        pokemon.forme_change(battle_ref2, ID::from(battle_only_species), Some(ID::from("zenmode")), false, "0", Some("[silent]"));
                    }
                }
            }
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (!pokemon.species.name.includes('Galar')) {
    ///         if (pokemon.species.id !== 'darmanitanzen') pokemon.formeChange('Darmanitan-Zen');
    ///     } else {
    ///         if (pokemon.species.id !== 'darmanitangalarzen') pokemon.formeChange('Darmanitan-Galar-Zen');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // if (!pokemon.species.name.includes('Galar'))
        let (species_name, species_id): (String, ID) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
                Some(data) => data,
                None => return EventResult::Continue,
            };

            (species_data.name.clone(), pokemon.species_id.clone())
        };

        if !species_name.contains("Galar") {
            // if (pokemon.species.id !== 'darmanitanzen') pokemon.formeChange('Darmanitan-Zen');
            if species_id.as_str() != "darmanitanzen" {
                unsafe {
                    let battle_ptr = battle as *mut Battle;
                    let battle_ref1 = &mut *battle_ptr;
                    let battle_ref2 = &mut *battle_ptr;

                    let side = &mut battle_ref1.sides[pokemon_pos.0];
                    let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                    if let Some(pokemon_index) = active_slot {
                        if pokemon_index < side.pokemon.len() {
                            let pokemon = &mut side.pokemon[pokemon_index];
                            pokemon.forme_change(battle_ref2, ID::from("darmanitanzen"), Some(ID::from("zenmode")), false, "0", None);
                        }
                    }
                }
            }
        } else {
            // if (pokemon.species.id !== 'darmanitangalarzen') pokemon.formeChange('Darmanitan-Galar-Zen');
            if species_id.as_str() != "darmanitangalarzen" {
                unsafe {
                    let battle_ptr = battle as *mut Battle;
                    let battle_ref1 = &mut *battle_ptr;
                    let battle_ref2 = &mut *battle_ptr;

                    let side = &mut battle_ref1.sides[pokemon_pos.0];
                    let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                    if let Some(pokemon_index) = active_slot {
                        if pokemon_index < side.pokemon.len() {
                            let pokemon = &mut side.pokemon[pokemon_index];
                            pokemon.forme_change(battle_ref2, ID::from("darmanitangalarzen"), Some(ID::from("zenmode")), false, "0", None);
                        }
                    }
                }
            }
        }

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     if (['Zen', 'Galar-Zen'].includes(pokemon.species.forme)) {
    ///         pokemon.formeChange(pokemon.species.battleOnly as string);
    ///     }
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // if (['Zen', 'Galar-Zen'].includes(pokemon.species.forme))
        let (forme, battle_only): (Option<String>, Option<String>) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
                Some(data) => data,
                None => return EventResult::Continue,
            };

            // Extract first string from StringOrVec
            let battle_only_str = match &species_data.battle_only {
                crate::dex::StringOrVec::Single(s) => Some(s.clone()),
                crate::dex::StringOrVec::Multiple(v) if !v.is_empty() => Some(v[0].clone()),
                _ => None,
            };

            (species_data.forme.clone(), battle_only_str)
        };

        if forme.as_deref() == Some("Zen") || forme.as_deref() == Some("Galar-Zen") {
            // pokemon.formeChange(pokemon.species.battleOnly as string);
            if let Some(battle_only_species) = battle_only {
                unsafe {
                    let battle_ptr = battle as *mut Battle;
                    let battle_ref1 = &mut *battle_ptr;
                    let battle_ref2 = &mut *battle_ptr;

                    let side = &mut battle_ref1.sides[pokemon_pos.0];
                    let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                    if let Some(pokemon_index) = active_slot {
                        if pokemon_index < side.pokemon.len() {
                            let pokemon = &mut side.pokemon[pokemon_index];
                            pokemon.forme_change(battle_ref2, ID::from(battle_only_species), Some(ID::from("zenmode")), false, "0", None);
                        }
                    }
                }
            }
        }

        EventResult::Continue
    }
}
