//! Grassy Seed Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onStart(pokemon) {
///     if (!pokemon.ignoringItem() && this.field.isTerrain('grassyterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon.ignoringItem() && this.field.isTerrain('grassyterrain'))
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let ignoring_item = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        pokemon.ignoring_item(battle, false)
    } else {
        return EventResult::Continue;
    };

    if ignoring_item {
        return EventResult::Continue;
    }

    // this.field.isTerrain('grassyterrain')
    let is_grassy_terrain = battle.field.terrain.as_str() == "grassyterrain";

    if is_grassy_terrain {
        // pokemon.useItem();
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    EventResult::Continue
}

/// onTerrainChange(pokemon) {
///     if (this.field.isTerrain('grassyterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_terrain_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.field.isTerrain('grassyterrain'))
    let is_grassy_terrain = battle.field.terrain.as_str() == "grassyterrain";

    if is_grassy_terrain {
        // pokemon.useItem();
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    EventResult::Continue
}
