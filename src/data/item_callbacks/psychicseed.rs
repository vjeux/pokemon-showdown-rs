//! Psychic Seed Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (!pokemon.ignoringItem() && this.field.isTerrain('psychicterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!pokemon.ignoringItem() && this.field.isTerrain('psychicterrain')) { pokemon.useItem(); }
    let should_use_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        !pokemon.ignoring_item(battle, false) && battle.field.terrain.as_str() == "psychicterrain"
    };

    if should_use_item {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.use_item();
    }

    EventResult::Continue
}

/// onTerrainChange(pokemon) {
///     if (this.field.isTerrain('psychicterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_terrain_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.field.isTerrain('psychicterrain')) { pokemon.useItem(); }
    if battle.field.terrain.as_str() == "psychicterrain" {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.use_item();
    }

    EventResult::Continue
}
