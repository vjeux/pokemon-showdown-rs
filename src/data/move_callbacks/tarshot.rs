//! Tar Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};



// Condition handlers
pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.terastallized) return false;
    ///     this.add('-start', pokemon, 'Tar Shot');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onEffectiveness(typeMod, target, type, move) {
    ///     if (move.type !== 'Fire') return;
    ///     if (!target) return;
    ///     if (type !== target.getTypes()[0]) return;
    ///     return typeMod + 1;
    /// }
    pub fn on_effectiveness(battle: &mut Battle, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
