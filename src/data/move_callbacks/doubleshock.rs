//! Double Shock Move
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

/// onTryMove(pokemon, target, move) {
///     if (pokemon.hasType('Electric')) return;
///     this.add('-fail', pokemon, 'move: Double Shock');
///     this.attrLastMove('[still]');
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

