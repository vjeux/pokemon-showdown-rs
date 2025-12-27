//! Recycle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

/// onHit(pokemon, source, move) {
///     if (pokemon.item || !pokemon.lastItem) return false;
///     const item = pokemon.lastItem;
///     pokemon.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
///     pokemon.setItem(item, source, move);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

