//! Nature Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, pokemon) {
///     let move = 'triattack';
///     if (this.field.isTerrain('electricterrain')) {
///         move = 'thunderbolt';
///     } else if (this.field.isTerrain('grassyterrain')) {
///         move = 'energyball';
///     } else if (this.field.isTerrain('mistyterrain')) {
///         move = 'moonblast';
///     } else if (this.field.isTerrain('psychicterrain')) {
///         move = 'psychic';
///     }
///     this.actions.useMove(move, pokemon, { target });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

