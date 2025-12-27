//! Terrain Pulse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyType(move, pokemon) {
///     if (!pokemon.isGrounded()) return;
///     switch (this.field.terrain) {
///     case 'electricterrain':
///         move.type = 'Electric';
///         break;
///     case 'grassyterrain':
///         move.type = 'Grass';
///         break;
///     case 'mistyterrain':
///         move.type = 'Fairy';
///         break;
///     case 'psychicterrain':
///         move.type = 'Psychic';
///         break;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onModifyMove(move, pokemon) {
///     if (this.field.terrain && pokemon.isGrounded()) {
///         move.basePower *= 2;
///         this.debug('BP doubled in Terrain');
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

