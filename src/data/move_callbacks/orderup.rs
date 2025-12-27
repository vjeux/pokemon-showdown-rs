//! Order Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (!pokemon.volatiles['commanded']) return;
///     const tatsugiri = pokemon.volatiles['commanded'].source;
///     if (tatsugiri.baseSpecies.baseSpecies !== 'Tatsugiri') return; // Should never happen
///     switch (tatsugiri.baseSpecies.forme) {
///     case 'Droopy':
///         this.boost({ def: 1 }, pokemon, pokemon);
///         break;
///     case 'Stretchy':
///         this.boost({ spe: 1 }, pokemon, pokemon);
///         break;
///     default:
///         this.boost({ atk: 1 }, pokemon, pokemon);
///         break;
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

