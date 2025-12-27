//! Roost Move
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

    /// onStart(target) {
    ///     if (target.terastallized) {
    ///         if (target.hasType('Flying')) {
    ///             this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
    ///         }
    ///         return false;
    ///     }
    ///     this.add('-singleturn', target, 'move: Roost');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onType(types, pokemon) {
    ///     this.effectState.typeWas = types;
    ///     return types.filter(type => type !== 'Flying');
    /// }
    pub fn on_type(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
