//! Mimicry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
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
pub fn on_terrain_change(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

