//! Shore Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(pokemon) {            let factor = 0.5;
///             if (this.field.isWeather('sandstorm')) {
///                 factor = 0.667;
///             }
///             const success = !!this.heal(this.modify(pokemon.maxhp, factor));
///             if (!success) {
///                 this.add('-fail', pokemon, 'heal');
///                 return this.NOT_FAIL;
///             }
///             return success;
///         }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

