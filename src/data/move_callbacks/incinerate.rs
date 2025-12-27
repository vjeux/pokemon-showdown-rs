//! Incinerate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon, source) {
///     const item = pokemon.getItem();
///     if ((item.isBerry || item.isGem) && pokemon.takeItem(source)) {
///         this.add('-enditem', pokemon, item.name, '[from] move: Incinerate');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

