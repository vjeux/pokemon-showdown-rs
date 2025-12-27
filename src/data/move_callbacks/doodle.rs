//! Doodle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(target, source, move) {
///     let success: boolean | null = false;
///     if (!target.getAbility().flags['failroleplay']) {
///         for (const pokemon of source.alliesAndSelf()) {
///             if (pokemon.ability === target.ability || pokemon.getAbility().flags['cantsuppress']) continue;
///             const oldAbility = pokemon.setAbility(target.ability, null, move);
///             if (oldAbility) {
///                 success = true;
///             } else if (!success && oldAbility === null) {
///                 success = null;
///             }
///         }
///     }
///     if (!success) {
///         if (success === false) {
///             this.add('-fail', source);
///         }
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

