//! Misty Seed Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onStart(pokemon) {
///     if (!pokemon.ignoringItem() && this.field.isTerrain('mistyterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon.ignoringItem() && this.field.isTerrain('mistyterrain'))
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let should_use_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        !pokemon.ignoring_item(battle, false) && battle.field.terrain.as_str() == "mistyterrain"
    };

    if should_use_item {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    EventResult::Continue
}

/// onTerrainChange(pokemon) {
///     if (this.field.isTerrain('mistyterrain')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_terrain_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.field.isTerrain('mistyterrain')) { pokemon.useItem(); }
    if battle.field.terrain.as_str() == "mistyterrain" {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    EventResult::Continue
}
