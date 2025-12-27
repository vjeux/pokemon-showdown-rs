//! Recycle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon, source, move) {
///     if (pokemon.item || !pokemon.lastItem) return false;
///     const item = pokemon.lastItem;
///     pokemon.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
///     pokemon.setItem(item, source, move);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

