//! Booster Energy Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onStart(pokemon) {
///     this.effectState.started = true;
///     ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // this.effectState.started = true;
    battle.with_effect_state(|state| {
        state.started = Some(true);
    });

    // ((this.effect as any).onUpdate as (p: Pokemon) => void).call(this, pokemon);
    // Call onUpdate with the pokemon
    if let Some(pos) = target_pos {
        on_update(battle, pos);
    }

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (!this.effectState.started || pokemon.transformed) return;
///
///     if (pokemon.hasAbility('protosynthesis') && !this.field.isWeather('sunnyday') && pokemon.useItem()) {
///         pokemon.addVolatile('protosynthesis');
///     }
///     if (pokemon.hasAbility('quarkdrive') && !this.field.isTerrain('electricterrain') && pokemon.useItem()) {
///         pokemon.addVolatile('quarkdrive');
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.started || pokemon.transformed) return;
    let (started, transformed) = {
        let started = battle.with_effect_state_ref(|state| {
            state.started.unwrap_or(false)
        }).unwrap_or(false);

        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (started, pokemon.transformed)
    };

    if !started || transformed {
        return EventResult::Continue;
    }

    // if (pokemon.hasAbility('protosynthesis') && !this.field.isWeather('sunnyday') && pokemon.useItem()) {
    //     pokemon.addVolatile('protosynthesis');
    // }
    let has_protosynthesis = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["protosynthesis"])
    };

    if has_protosynthesis {
        use crate::dex_data::ID;
        let is_sunny = battle.is_weather("sunnyday");

        if !is_sunny {
            let used_item = {
                let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                Pokemon::use_item(battle, pokemon_pos, None, None).is_some()
            };

            if used_item {
                Pokemon::add_volatile(battle, pokemon_pos, ID::from("protosynthesis"), None, None, None, None);
            }
        }
    }

    // if (pokemon.hasAbility('quarkdrive') && !this.field.isTerrain('electricterrain') && pokemon.useItem()) {
    //     pokemon.addVolatile('quarkdrive');
    // }
    let has_quarkdrive = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["quarkdrive"])
    };

    if has_quarkdrive {
        use crate::dex_data::ID;
        let is_electric_terrain = battle.is_terrain("electricterrain");

        if !is_electric_terrain {
            let used_item = {
                let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                Pokemon::use_item(battle, pokemon_pos, None, None).is_some()
            };

            if used_item {
                Pokemon::add_volatile(battle, pokemon_pos, ID::from("quarkdrive"), None, None, None, None);
            }
        }
    }

    EventResult::Continue
}

/// onTakeItem(item, source) {
///     if (source.baseSpecies.tags.includes("Paradox")) return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.tags.includes("Paradox")) return false;
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            let source_species = battle.dex.species().get(source_pokemon.base_species.as_str());
            if let Some(species_data) = source_species {
                if species_data.tags.contains(&"Paradox".to_string()) {
                    // return false;
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
