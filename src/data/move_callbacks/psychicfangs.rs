//! Psychic Fangs Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit(pokemon) {            // will shatter screens through sub, before you hit
///             pokemon.side.removeSideCondition('reflect');
///             pokemon.side.removeSideCondition('lightscreen');
///             pokemon.side.removeSideCondition('auroraveil');
///         }
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

