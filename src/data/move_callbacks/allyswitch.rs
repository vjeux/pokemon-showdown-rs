//! Ally Switch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onPrepareHit(pokemon) {            return pokemon.addVolatile('allyswitch');
///         }
pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(pokemon) {            let success = true;
///             // Fail in formats where you don't control allies
///             if (this.format.gameType !== 'doubles' && this.format.gameType !== 'triples') success = false;
/// 
///             // Fail in triples if the Pokemon is in the middle
///             if (pokemon.side.active.length === 3 && pokemon.position === 1) success = false;
/// 
///             const newPosition = (pokemon.position === 0 ? pokemon.side.active.length - 1 : 0);
///             if (!pokemon.side.active[newPosition]) success = false;
///             if (pokemon.side.active[newPosition].fainted) success = false;
///             if (!success) {
///                 this.add('-fail', pokemon, 'move: Ally Switch');
///                 this.attrLastMove('[still]');
///                 return this.NOT_FAIL;
///             }
///             this.swapPosition(pokemon, newPosition, '[from] move: Ally Switch');
///         }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
